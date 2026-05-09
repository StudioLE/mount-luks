//! Handler for the `validate` subcommand.

use crate::prelude::*;

const TOTAL_STEPS: usize = 4;

/// Handler for validating a LUKS key against a partition.
pub struct ValidateHandler {
    /// Adapter for checking root privileges.
    is_root: Arc<dyn IsRoot>,
    /// Adapter for resolving PARTUUID to a device path.
    resolve_partition: Arc<dyn ResolvePartition>,
    /// Adapter for checking if a key is valid for a LUKS partition.
    check_key: Arc<dyn CheckKey>,
    /// Adapter for retrieving the composite key.
    get_key: Arc<dyn GetKey>,
}

impl FromServices for ValidateHandler {
    type Error = ResolveError;
    fn from_services(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        Ok(Self {
            is_root: services.get_trait::<dyn IsRoot>()?,
            resolve_partition: services.get_trait::<dyn ResolvePartition>()?,
            check_key: services.get_trait::<dyn CheckKey>()?,
            get_key: services.get_trait::<dyn GetKey>()?,
        })
    }
}

impl ValidateHandler {
    /// Execute the validate workflow.
    pub fn execute(&self) -> Result<(), StructuredError> {
        let mut progress = Progress::new(TOTAL_STEPS);

        progress.step("Checking if root");
        self.is_root.is_root().map_err(StructuredError::from)?;
        progress.ok("Access granted");

        progress.step("Resolving partition");
        let partition = self
            .resolve_partition
            .resolve_partition()
            .map_err(StructuredError::from)?;
        progress.ok(&format!("Resolved to {}", partition.display()));

        progress.step("Getting key");
        let key = self.get_key.get().map_err(StructuredError::from)?;
        progress.ok("Key retrieved");

        progress.step("Validating key against partition");
        self.check_key
            .check_key(&partition, &key)
            .map_err(StructuredError::from)?;
        progress.ok("Key is valid");

        Ok(())
    }
}

#[cfg(test)]
impl ValidateHandler {
    /// Create a [`ValidateHandler`] with all adapters succeeding by default.
    pub fn mock() -> Self {
        let mut is_root = MockIsRoot::new();
        is_root.expect_is_root().returning(|| Ok(()));
        let mut resolve_partition = MockResolvePartition::new();
        resolve_partition
            .expect_resolve_partition()
            .returning(|| Ok(PathBuf::from("/dev/fake-partition")));
        let mut check_key = MockCheckKey::new();
        check_key.expect_check_key().returning(|_, _| Ok(()));
        let mut get_key = MockGetKey::new();
        get_key
            .expect_get()
            .returning(|| Ok(String::from("test-key")));
        Self {
            is_root: Arc::new(is_root),
            resolve_partition: Arc::new(resolve_partition),
            check_key: Arc::new(check_key),
            get_key: Arc::new(get_key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
