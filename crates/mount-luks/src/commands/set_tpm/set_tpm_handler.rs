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
    pub fn execute(&self) -> Result<(), Report<SetTpmError>> {
        let mut progress = Progress::new(TOTAL_STEPS);

        progress.step("Checking if root");
        self.is_root
            .is_root()
            .change_context(SetTpmError::CheckRoot)?;
        progress.ok("Access granted");

        let handle = self
            .options
            .tpm_handle
            .ok_or_else(|| Report::new(SetTpmError::HandleRequired))?;

        progress.step("Checking TPM handle availability");
        let handles = self
            .get_handles
            .get_handles()
            .change_context(SetTpmError::GetHandles)?;
        if handles.contains(&handle) {
            return Err(Report::new(SetTpmError::HandleInUse).attach("Handle", handle));
        }
        progress.ok("TPM handle is available");

        progress.step("Creating TPM policy");
        self.create_policy
            .create_policy()
            .change_context(SetTpmError::CreatePolicy)?;
        progress.ok("TPM policy created");

        progress.step("Creating TPM primary key");
        self.create_primary
            .create_primary()
            .change_context(SetTpmError::CreatePrimary)?;
        progress.ok("TPM primary key created");

        progress.step("Getting key to seal");
        let key = self
            .prompt
            .prompt("Enter key to seal into TPM: ")
            .change_context(SetTpmError::Prompt)?;
        progress.ok("Key retrieved");

        progress.step("Creating TPM object");
        self.create_object
            .create_object(key.trim())
            .change_context(SetTpmError::CreateObject)?;
        progress.ok("TPM object created");

        progress.step("Loading TPM object");
        self.load_object
            .load_object()
            .change_context(SetTpmError::LoadObject)?;
        progress.ok("TPM object loaded");

        progress.step("Persisting TPM object");
        self.persist
            .persist(handle)
            .change_context(SetTpmError::Persist)?;
        progress.ok("TPM object persisted");

        Ok(())
    }
}

/// Errors returned by [`SetTpmHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum SetTpmError {
    /// Unable to confirm the process is running as root.
    #[error("Unable to confirm root privileges")]
    CheckRoot,
    /// No TPM handle is configured.
    #[error("TPM handle is required")]
    HandleRequired,
    /// Unable to list the persistent TPM handles.
    #[error("Unable to get TPM handles")]
    GetHandles,
    /// The TPM handle already holds an object.
    #[error("TPM handle is already in use")]
    HandleInUse,
    /// Unable to create the TPM policy.
    #[error("Unable to create TPM policy")]
    CreatePolicy,
    /// Unable to create the TPM primary key.
    #[error("Unable to create TPM primary key")]
    CreatePrimary,
    /// Unable to read the key to seal.
    #[error("Unable to read key to seal")]
    Prompt,
    /// Unable to create the sealed TPM object.
    #[error("Unable to create TPM object")]
    CreateObject,
    /// Unable to load the TPM object.
    #[error("Unable to load TPM object")]
    LoadObject,
    /// Unable to persist the TPM object.
    #[error("Unable to persist TPM object")]
    Persist,
}

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
        assert_eq!(report.current_context(), &SetTpmError::HandleInUse);
    }

    /// [`SetTpmHandler::execute`] when the TPM policy can't be created.
    #[test]
    fn set_tpm_handler_execute_create_policy_error() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(CreatePolicyStub {
                result: Err(CreatePolicyError::Failed),
            })
            .build();
        let handler = services.expect::<SetTpmHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to seal without a policy");
        assert_eq!(report.current_context(), &SetTpmError::CreatePolicy);
    }
}
