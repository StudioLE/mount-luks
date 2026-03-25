//! Handler for the `validate` subcommand.

use crate::prelude::*;

/// Handler for validating a LUKS key against a partition.
pub struct ValidateHandler {
    /// Configuration options for the validate operation.
    options: Arc<Options>,
    /// Adapter for checking root privileges.
    is_root: Arc<dyn IsRoot>,
    /// Adapter for checking if a key is valid for a LUKS partition.
    check_key: Arc<dyn CheckKey>,
    /// Adapter for retrieving the composite key.
    get_key: Arc<dyn GetKey>,
}

impl FromServices for ValidateHandler {
    type Error = ResolveError;
    fn from_services(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        Ok(Self {
            options: services.get::<Options>()?,
            is_root: services.get_trait::<dyn IsRoot>()?,
            check_key: services.get_trait::<dyn CheckKey>()?,
            get_key: services.get_trait::<dyn GetKey>()?,
        })
    }
}

impl ValidateHandler {
    /// Execute the validate workflow.
    pub fn execute(&self) -> Result<(), StructuredError> {
        let counter = Mutex::new(0);
        let total_steps = 3;

        print_step_start(&counter, total_steps, "Checking if root");
        self.is_root.is_root().map_err(StructuredError::from)?;
        print_step_completed("Access granted");

        print_step_start(&counter, total_steps, "Getting key");
        let key = self.get_key.get().map_err(StructuredError::from)?;
        print_step_completed("Key retrieved");

        print_step_start(&counter, total_steps, "Validating key against partition");
        self.check_key
            .check_key(&self.options.partition_path, &key)
            .map_err(StructuredError::from)?;
        print_step_completed("Key is valid");

        Ok(())
    }
}

#[cfg(test)]
impl ValidateHandler {
    /// Create a [`ValidateHandler`] with all adapters succeeding by default.
    pub fn mock() -> Self {
        let options = tests::make_options();
        let mut is_root = MockIsRoot::new();
        is_root.expect_is_root().returning(|| Ok(()));
        let mut check_key = MockCheckKey::new();
        check_key.expect_check_key().returning(|_, _| Ok(()));
        let mut get_key = MockGetKey::new();
        get_key
            .expect_get()
            .returning(|| Ok(String::from("test-key")));
        Self {
            options,
            is_root: Arc::new(is_root),
            check_key: Arc::new(check_key),
            get_key: Arc::new(get_key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn make_options() -> Arc<Options> {
        Arc::new(Options {
            partition_path: PathBuf::from("/dev/fake"),
            key_prompt: Some(true),
            ..Options::default()
        })
    }

    #[test]
    fn validate_succeeds_with_valid_key() {
        // Arrange
        let handler = ValidateHandler::mock();

        // Act
        let result = handler.execute();

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn validate_fails_with_invalid_key() {
        // Arrange
        let mut check_key = MockCheckKey::new();
        check_key
            .expect_check_key()
            .returning(|_, _| Err(Report::new(CheckKeyError)));
        let handler = ValidateHandler {
            check_key: Arc::new(check_key),
            ..ValidateHandler::mock()
        };

        // Act
        let result = handler.execute();

        // Assert
        let error = result.expect_err("should be an error");
        let error = error
            .current_context()
            .downcast_ref::<CheckKeyError>()
            .expect("should be able to downcast");
        assert_eq!(error, &CheckKeyError);
    }

    #[test]
    fn validate_fails_when_no_key_sources() {
        // Arrange
        let mut get_key = MockGetKey::new();
        get_key
            .expect_get()
            .returning(|| Err(Report::new(GetKeyError::Required)));
        let handler = ValidateHandler {
            get_key: Arc::new(get_key),
            ..ValidateHandler::mock()
        };

        // Act
        let result = handler.execute();

        // Assert
        let error = result.expect_err("should be an error");
        let error = error
            .current_context()
            .downcast_ref::<GetKeyError>()
            .expect("should be able to downcast");
        assert_eq!(error, &GetKeyError::Required);
    }
}
