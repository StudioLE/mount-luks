//! Handler for the `set-tpm` subcommand.

use crate::prelude::*;

/// Handler for sealing a key into the TPM.
pub struct SetTpmHandler {
    /// Configuration options for the set-tpm operation.
    options: Arc<Options>,
    /// Adapter for checking root privileges.
    is_root: Arc<dyn IsRoot>,
    /// Adapter for listing persistent TPM handles.
    get_handles: Arc<dyn GetHandles>,
    /// Adapter for creating a TPM PCR policy.
    create_policy: Arc<dyn CreatePolicy>,
    /// Adapter for creating a TPM primary key.
    create_primary: Arc<dyn CreatePrimary>,
    /// Adapter for creating a sealed TPM object.
    create_object: Arc<dyn CreateObject>,
    /// Adapter for loading a TPM object.
    load_object: Arc<dyn LoadObject>,
    /// Adapter for making a TPM object persistent.
    persist: Arc<dyn Persist>,
    /// Adapter for prompting for a password.
    prompt: Arc<dyn PromptPassword>,
}

impl FromServices for SetTpmHandler {
    type Error = ResolveError;
    fn from_services(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        Ok(Self {
            options: services.get::<Options>()?,
            is_root: services.get_trait::<dyn IsRoot>()?,
            get_handles: services.get_trait::<dyn GetHandles>()?,
            create_policy: services.get_trait::<dyn CreatePolicy>()?,
            create_primary: services.get_trait::<dyn CreatePrimary>()?,
            create_object: services.get_trait::<dyn CreateObject>()?,
            load_object: services.get_trait::<dyn LoadObject>()?,
            persist: services.get_trait::<dyn Persist>()?,
            prompt: services.get_trait::<dyn PromptPassword>()?,
        })
    }
}

impl SetTpmHandler {
    /// Execute the set-tpm workflow.
    pub fn execute(&self) -> Result<(), StructuredError> {
        let counter = Mutex::new(0);
        let total_steps = 8;

        print_step_start(&counter, total_steps, "Checking if root");
        self.is_root.is_root().map_err(StructuredError::from)?;
        print_step_completed("Access granted");

        let handle = self
            .options
            .tpm_handle
            .ok_or_else(|| StructuredError::new(HandleRequired))?;

        print_step_start(&counter, total_steps, "Checking TPM handle availability");
        let handles = self
            .get_handles
            .get_handles()
            .map_err(StructuredError::from)?;
        if handles.contains(&handle) {
            return Err(Report::new(HandleInUse).attach("Handle", handle).into());
        }
        print_step_completed("TPM handle is available");

        print_step_start(&counter, total_steps, "Creating TPM policy");
        self.create_policy
            .create_policy()
            .map_err(StructuredError::from)?;
        print_step_completed("TPM policy created");

        print_step_start(&counter, total_steps, "Creating TPM primary key");
        self.create_primary
            .create_primary()
            .map_err(StructuredError::from)?;
        print_step_completed("TPM primary key created");

        print_step_start(&counter, total_steps, "Getting key to seal");
        let key = self
            .prompt
            .prompt("Enter key to seal into TPM: ")
            .map_err(StructuredError::from)?;
        print_step_completed("Key retrieved");

        print_step_start(&counter, total_steps, "Creating TPM object");
        self.create_object
            .create_object(key.trim())
            .map_err(StructuredError::from)?;
        print_step_completed("TPM object created");

        print_step_start(&counter, total_steps, "Loading TPM object");
        self.load_object
            .load_object()
            .map_err(StructuredError::from)?;
        print_step_completed("TPM object loaded");

        print_step_start(&counter, total_steps, "Persisting TPM object");
        self.persist
            .persist(handle)
            .map_err(StructuredError::from)?;
        print_step_completed("TPM object persisted");

        Ok(())
    }
}

/// Error returned by [`SetTpmHandler::execute`] when no TPM handle is configured.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("TPM handle is required")]
pub struct HandleRequired;

/// Error returned by [`SetTpmHandler::execute`] when the TPM handle is already in use.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("TPM handle is already in use")]
pub struct HandleInUse;

#[cfg(test)]
impl SetTpmHandler {
    /// Create a [`SetTpmHandler`] with all adapters succeeding by default.
    pub fn mock() -> Self {
        let options = tests::make_options();
        let mut is_root = MockIsRoot::new();
        is_root.expect_is_root().returning(|| Ok(()));
        let mut get_handles = MockGetHandles::new();
        get_handles
            .expect_get_handles()
            .returning(|| Ok(Vec::new()));
        let mut create_policy = MockCreatePolicy::new();
        create_policy.expect_create_policy().returning(|| Ok(()));
        let mut create_primary = MockCreatePrimary::new();
        create_primary.expect_create_primary().returning(|| Ok(()));
        let mut create_object = MockCreateObject::new();
        create_object.expect_create_object().returning(|_| Ok(()));
        let mut load_object = MockLoadObject::new();
        load_object.expect_load_object().returning(|| Ok(()));
        let mut persist = MockPersist::new();
        persist.expect_persist().returning(|_| Ok(()));
        let mut prompt = MockPromptPassword::new();
        prompt
            .expect_prompt()
            .returning(|_| Ok(String::from("secret-key")));
        Self {
            options,
            is_root: Arc::new(is_root),
            get_handles: Arc::new(get_handles),
            create_policy: Arc::new(create_policy),
            create_primary: Arc::new(create_primary),
            create_object: Arc::new(create_object),
            load_object: Arc::new(load_object),
            persist: Arc::new(persist),
            prompt: Arc::new(prompt),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_handle() -> PersistentHandle {
        PersistentHandle::new(0x81000001).expect("valid handle")
    }

    pub fn make_options() -> Arc<Options> {
        Arc::new(Options {
            tpm_handle: Some(make_handle()),
            ..Options::default()
        })
    }

    #[test]
    fn set_tpm_succeeds() {
        // Arrange
        let handler = SetTpmHandler::mock();

        // Act
        let result = handler.execute();

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn set_tpm_fails_when_handle_in_use() {
        // Arrange
        let handle = make_handle();
        let mut get_handles = MockGetHandles::new();
        get_handles
            .expect_get_handles()
            .returning(move || Ok(vec![handle]));
        let handler = SetTpmHandler {
            get_handles: Arc::new(get_handles),
            ..SetTpmHandler::mock()
        };

        // Act
        let result = handler.execute();

        // Assert
        let error = result.expect_err("should be an error");
        let error = error
            .current_context()
            .downcast_ref::<HandleInUse>()
            .expect("should be able to downcast");
        assert_eq!(error, &HandleInUse);
    }

    #[test]
    fn set_tpm_fails_when_create_policy_fails() {
        // Arrange
        let mut create_policy = MockCreatePolicy::new();
        create_policy
            .expect_create_policy()
            .returning(|| Err(Report::new(CreatePolicyError)));
        let handler = SetTpmHandler {
            create_policy: Arc::new(create_policy),
            ..SetTpmHandler::mock()
        };

        // Act
        let result = handler.execute();

        // Assert
        let error = result.expect_err("should be an error");
        let error = error
            .current_context()
            .downcast_ref::<CreatePolicyError>()
            .expect("should be able to downcast");
        assert_eq!(error, &CreatePolicyError);
    }
}
