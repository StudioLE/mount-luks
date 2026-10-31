//! Handler for the `mount` subcommand.

use crate::prelude::*;

const TOTAL_STEPS: usize = 8;

/// Handler for unlocking and mounting a LUKS partition.
#[derive(FromServices)]
pub struct MountHandler {
    /// Configuration options for the mount operation.
    options: Arc<Options>,
    /// Adapter for checking root privileges.
    check_root: Arc<dyn CheckRoot>,
    /// Adapter for checking if a partition is LUKS encrypted.
    check_luks: Arc<dyn CheckLuks>,
    /// Adapter for unlocking a LUKS partition.
    unlock: Arc<dyn Unlock>,
    /// Adapter for checking if a path is already mounted.
    check_not_mounted: Arc<dyn CheckNotMounted>,
    /// Adapter for mounting a partition.
    mount_partition: Arc<dyn MountPartition>,
    /// Adapter for retrieving the composite key.
    get_key: Arc<dyn GetKey>,
    /// Adapter for resolving PARTUUID to a device path.
    resolve_partition: Arc<dyn ResolvePartition>,
    /// Adapter for checking mount point existence.
    check_mount: Arc<dyn CheckMountExists>,
    /// Adapter for checking if the partition is locked.
    check_partition_locked: Arc<dyn CheckPartitionLocked>,
}

impl MountHandler {
    /// Execute the mount workflow.
    pub fn execute(&self) -> Result<(), Report<MountError>> {
        let mut progress = Progress::new(TOTAL_STEPS);

        progress.step("Checking if root");
        self.check_root
            .check_root()
            .change_context(MountError::CheckRoot)?;
        progress.ok("Access granted");

        progress.step("Resolving partition");
        let partition = self
            .resolve_partition
            .resolve_partition()
            .change_context(MountError::ResolvePartition)?;
        progress.ok(&format!("Resolved to {}", partition.display()));

        progress.step("Checking if partition is encrypted with LUKS");
        self.check_luks
            .check_luks(&partition)
            .change_context(MountError::CheckLuks)?;
        progress.ok("Partition is encrypted with LUKS");

        progress.step("Checking if partition is already unlocked");
        self.check_partition_locked
            .check_partition_locked()
            .change_context(MountError::CheckLocked)?;
        progress.ok("Partition is locked");

        progress.step("Unlocking LUKS partition");
        let key = self.get_key.get_key().change_context(MountError::GetKey)?;
        self.unlock
            .unlock(&partition, &self.options.mapper_name, &key)
            .change_context(MountError::Unlock)?;
        progress.ok("Unlocked LUKS partition");

        progress.step("Checking mount point exists");
        self.check_mount
            .check_mount_exists()
            .change_context(MountError::CheckMountPoint)?;
        progress.ok("Mount point exists");

        progress.step("Checking if already mounted");
        self.check_not_mounted
            .check_not_mounted(&self.options.mount_path)
            .change_context(MountError::CheckMounted)?;
        progress.ok("Partition is not mounted");

        progress.step("Mounting partition");
        let mapper_path = self.options.get_mapper_path();
        self.mount_partition
            .mount_partition(&mapper_path, &self.options.mount_path)
            .change_context(MountError::Mount)?;
        progress.ok("Partition mounted successfully");

        Ok(())
    }
}

/// Errors returned by [`MountHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum MountError {
    /// Unable to confirm the process is running as root.
    #[error("Unable to confirm root privileges")]
    CheckRoot,
    /// Unable to resolve the PARTUUID to a device path.
    #[error("Unable to resolve partition")]
    ResolvePartition,
    /// Unable to confirm the partition is encrypted with LUKS.
    #[error("Unable to confirm partition is encrypted with LUKS")]
    CheckLuks,
    /// Unable to confirm the partition is locked.
    #[error("Unable to confirm partition is locked")]
    CheckLocked,
    /// Unable to get the key.
    #[error("Unable to get key")]
    GetKey,
    /// Unable to unlock the partition.
    #[error("Unable to unlock partition")]
    Unlock,
    /// Unable to confirm the mount point exists.
    #[error("Unable to confirm mount point exists")]
    CheckMountPoint,
    /// Unable to confirm the partition isn't mounted.
    #[error("Unable to confirm partition is not mounted")]
    CheckMounted,
    /// Unable to mount the partition.
    #[error("Unable to mount partition")]
    Mount,
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
            .with_instance(CheckLuksStub {
                result: Err(CheckLuksError::NotLuks),
            })
            .build();
        let handler = services.expect::<MountHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to mount without LUKS");
        assert_eq!(report.current_context(), &MountError::CheckLuks);
    }

    /// [`MountHandler::execute`] when the mount path is already mounted.
    #[test]
    fn mount_handler_execute_already_mounted() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(CheckNotMountedStub {
                result: Err(CheckNotMountedError::AlreadyMounted),
            })
            .build();
        let handler = services.expect::<MountHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to mount twice");
        assert_eq!(report.current_context(), &MountError::CheckMounted);
    }

    /// [`MountHandler::execute`] when the partition can't be unlocked.
    #[test]
    fn mount_handler_execute_unlock_error() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(UnlockStub {
                result: Err(UnlockError::Failed),
            })
            .build();
        let handler = services.expect::<MountHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to mount when locked");
        assert_eq!(report.current_context(), &MountError::Unlock);
    }
}
