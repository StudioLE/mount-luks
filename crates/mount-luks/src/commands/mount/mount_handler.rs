//! Handler for the `mount` subcommand.

use crate::prelude::*;

const TOTAL_STEPS: usize = 8;

/// Handler for unlocking and mounting a LUKS partition.
#[derive(FromServices)]
pub struct MountHandler {
    /// Configuration options for the mount operation.
    options: Arc<Options>,
    /// Adapter for checking root privileges.
    is_root: Arc<dyn IsRoot>,
    /// Adapter for checking if a partition is LUKS encrypted.
    is_luks: Arc<dyn IsLuks>,
    /// Adapter for unlocking a LUKS partition.
    unlock: Arc<dyn Unlock>,
    /// Adapter for checking if a path is already mounted.
    find_mount: Arc<dyn FindMount>,
    /// Adapter for mounting a partition.
    mount: Arc<dyn MountPartition>,
    /// Adapter for retrieving the composite key.
    get_key: Arc<dyn GetKey>,
    /// Adapter for resolving PARTUUID to a device path.
    resolve_partition: Arc<dyn ResolvePartition>,
    /// Adapter for checking mount point existence.
    check_mount: Arc<dyn CheckMountExists>,
    /// Adapter for checking if the partition is locked.
    is_locked: Arc<dyn IsPartitionLocked>,
}

impl MountHandler {
    /// Execute the mount workflow.
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

        progress.step("Checking if partition is encrypted with LUKS");
        self.is_luks
            .is_luks(&partition)
            .map_err(StructuredError::from)?;
        progress.ok("Partition is encrypted with LUKS");

        progress.step("Checking if partition is already unlocked");
        self.is_locked
            .is_partition_locked()
            .map_err(StructuredError::from)?;
        progress.ok("Partition is locked");

        progress.step("Unlocking LUKS partition");
        let key = self.get_key.get().map_err(StructuredError::from)?;
        self.unlock
            .unlock(&partition, &self.options.mapper_name, &key)
            .map_err(StructuredError::from)?;
        progress.ok("Unlocked LUKS partition");

        progress.step("Checking mount point exists");
        self.check_mount
            .check_mount_exists()
            .map_err(StructuredError::from)?;
        progress.ok("Mount point exists");

        progress.step("Checking if already mounted");
        self.find_mount
            .check_not_mounted(&self.options.mount_path)
            .map_err(StructuredError::from)?;
        progress.ok("Partition is not mounted");

        progress.step("Mounting partition");
        let mapper_path = self.options.get_mapper_path();
        self.mount
            .mount(&mapper_path, &self.options.mount_path)
            .map_err(StructuredError::from)?;
        progress.ok("Partition mounted successfully");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`MountHandler::execute`] with every step succeeding.
    #[test]
    fn mount_handler_execute() {
        // Arrange
        let services = ServiceBuilder::new().with_test_services().build();
        let handler = services.expect::<MountHandler>();
        // Act
        let output = handler.execute();
        // Assert
        assert!(output.is_ok());
    }

    /// [`MountHandler::execute`] when the partition isn't encrypted with LUKS.
    #[test]
    fn mount_handler_execute_not_luks() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(IsLuksStub {
                result: Err(IsLuksError::NotLuks),
            })
            .build();
        let handler = services.expect::<MountHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to mount without LUKS");
        let context = report.current_context().downcast_ref::<IsLuksError>();
        assert_eq!(context, Some(&IsLuksError::NotLuks));
    }

    /// [`MountHandler::execute`] when the mount path is already mounted.
    #[test]
    fn mount_handler_execute_already_mounted() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(FindMountStub {
                result: Err(AlreadyMounted),
            })
            .build();
        let handler = services.expect::<MountHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to mount twice");
        let context = report.current_context().downcast_ref::<AlreadyMounted>();
        assert_eq!(context, Some(&AlreadyMounted));
    }

    /// [`MountHandler::execute`] when the partition can't be unlocked.
    #[test]
    fn mount_handler_execute_unlock_error() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(UnlockStub {
                result: Err(UnlockError),
            })
            .build();
        let handler = services.expect::<MountHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to mount when locked");
        let context = report.current_context().downcast_ref::<UnlockError>();
        assert_eq!(context, Some(&UnlockError));
    }
}
