//! Handler for the `set-tpm` subcommand.

use crate::prelude::*;

const TOTAL_STEPS: usize = 8;

/// Handler for sealing a key into the TPM.
#[derive(FromServices)]
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

impl SetTpmHandler {
    /// Execute the set-tpm workflow.
    pub fn execute(&self) -> Result<(), StructuredError> {
        let mut progress = Progress::new(TOTAL_STEPS);

        progress.step("Checking if root");
        self.is_root.is_root().map_err(StructuredError::from)?;
        progress.ok("Access granted");

        let handle = self
            .options
            .tpm_handle
            .ok_or_else(|| StructuredError::new(HandleRequired))?;

        progress.step("Checking TPM handle availability");
        let handles = self
            .get_handles
            .get_handles()
            .map_err(StructuredError::from)?;
        if handles.contains(&handle) {
            return Err(Report::new(HandleInUse).attach("Handle", handle).into());
        }
        progress.ok("TPM handle is available");

        progress.step("Creating TPM policy");
        self.create_policy
            .create_policy()
            .map_err(StructuredError::from)?;
        progress.ok("TPM policy created");

        progress.step("Creating TPM primary key");
        self.create_primary
            .create_primary()
            .map_err(StructuredError::from)?;
        progress.ok("TPM primary key created");

        progress.step("Getting key to seal");
        let key = self
            .prompt
            .prompt("Enter key to seal into TPM: ")
            .map_err(StructuredError::from)?;
        progress.ok("Key retrieved");

        progress.step("Creating TPM object");
        self.create_object
            .create_object(key.trim())
            .map_err(StructuredError::from)?;
        progress.ok("TPM object created");

        progress.step("Loading TPM object");
        self.load_object
            .load_object()
            .map_err(StructuredError::from)?;
        progress.ok("TPM object loaded");

        progress.step("Persisting TPM object");
        self.persist
            .persist(handle)
            .map_err(StructuredError::from)?;
        progress.ok("TPM object persisted");

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
mod tests {
    use super::*;

    /// [`SetTpmHandler::execute`] with every step succeeding.
    #[test]
    fn set_tpm_handler_execute() {
        // Arrange
        let services = ServiceBuilder::new().with_test_services().build();
        let handler = services.expect::<SetTpmHandler>();
        // Act
        let output = handler.execute();
        // Assert
        assert!(output.is_ok());
    }

    /// [`SetTpmHandler::execute`] when the TPM handle is already in use.
    #[test]
    fn set_tpm_handler_execute_handle_in_use() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(GetHandlesStub {
                handles: Ok(vec![PersistentHandle::fixture()]),
            })
            .build();
        let handler = services.expect::<SetTpmHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to reuse a handle");
        let context = report.current_context().downcast_ref::<HandleInUse>();
        assert_eq!(context, Some(&HandleInUse));
    }

    /// [`SetTpmHandler::execute`] when the TPM policy can't be created.
    #[test]
    fn set_tpm_handler_execute_create_policy_error() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(CreatePolicyStub {
                result: Err(CreatePolicyError),
            })
            .build();
        let handler = services.expect::<SetTpmHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to seal without a policy");
        let context = report.current_context().downcast_ref::<CreatePolicyError>();
        assert_eq!(context, Some(&CreatePolicyError));
    }
}
