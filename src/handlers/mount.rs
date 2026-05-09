//! Handler for the `mount` subcommand.

use crate::prelude::*;

const TOTAL_STEPS: usize = 8;

/// Handler for unlocking and mounting a LUKS partition.
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

impl FromServices for MountHandler {
    type Error = ResolveError;
    fn from_services(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        Ok(Self {
            options: services.get::<Options>()?,
            is_root: services.get_trait::<dyn IsRoot>()?,
            is_luks: services.get_trait::<dyn IsLuks>()?,
            unlock: services.get_trait::<dyn Unlock>()?,
            find_mount: services.get_trait::<dyn FindMount>()?,
            mount: services.get_trait::<dyn MountPartition>()?,
            get_key: services.get_trait::<dyn GetKey>()?,
            resolve_partition: services.get_trait::<dyn ResolvePartition>()?,
            check_mount: services.get_trait::<dyn CheckMountExists>()?,
            is_locked: services.get_trait::<dyn IsPartitionLocked>()?,
        })
    }
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
impl MountHandler {
    /// Create a [`MountHandler`] with all adapters succeeding by default.
    pub fn mock() -> Self {
        let options = tests::make_options();
        let mut is_root = MockIsRoot::new();
        is_root.expect_is_root().returning(|| Ok(()));
        let mut is_luks = MockIsLuks::new();
        is_luks.expect_is_luks().returning(|_| Ok(()));
        let mut unlock = MockUnlock::new();
        unlock.expect_unlock().returning(|_, _, _| Ok(()));
        let mut find_mount = MockFindMount::new();
        find_mount.expect_check_not_mounted().returning(|_| Ok(()));
        let mut mount = MockMountPartition::new();
        mount.expect_mount().returning(|_, _| Ok(()));
        let mut get_key = MockGetKey::new();
        get_key
            .expect_get()
            .returning(|| Ok(String::from("test-key")));
        let mut resolve_partition = MockResolvePartition::new();
        resolve_partition
            .expect_resolve_partition()
            .returning(|| Ok(PathBuf::from("/dev/fake-partition")));
        let mut check_mount = MockCheckMountExists::new();
        check_mount.expect_check_mount_exists().returning(|| Ok(()));
        let mut is_locked = MockIsPartitionLocked::new();
        is_locked.expect_is_partition_locked().returning(|| Ok(()));
        Self {
            options,
            is_root: Arc::new(is_root),
            is_luks: Arc::new(is_luks),
            unlock: Arc::new(unlock),
            find_mount: Arc::new(find_mount),
            mount: Arc::new(mount),
            get_key: Arc::new(get_key),
            resolve_partition: Arc::new(resolve_partition),
            check_mount: Arc::new(check_mount),
            is_locked: Arc::new(is_locked),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn make_options() -> Arc<Options> {
        Arc::new(Options {
            partuuid: String::from("fake-uuid"),
            mapper_name: String::from("fake-mapper"),
            mount_path: PathBuf::from("/mnt/fake-mount"),
            key_prompt: Some(true),
            ..Options::default()
        })
    }

    #[test]
    fn mount_succeeds_with_all_steps() {
        // Arrange
        let handler = MountHandler::mock();

        // Act
        let result = handler.execute();

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn mount_fails_when_not_luks() {
        // Arrange
        let mut is_luks = MockIsLuks::new();
        is_luks
            .expect_is_luks()
            .returning(|_| Err(Report::new(IsLuksError::NotLuks)));
        let handler = MountHandler {
            is_luks: Arc::new(is_luks),
            ..MountHandler::mock()
        };

        // Act
        let result = handler.execute();

        // Assert
        let error = result.expect_err("should be an error");
        let error = error
            .current_context()
            .downcast_ref::<IsLuksError>()
            .expect("should be able to downcast");
        assert_eq!(error, &IsLuksError::NotLuks);
    }

    #[test]
    fn mount_fails_when_already_mounted() {
        // Arrange
        let mut find_mount = MockFindMount::new();
        find_mount
            .expect_check_not_mounted()
            .returning(|_| Err(Report::new(AlreadyMounted)));
        let handler = MountHandler {
            find_mount: Arc::new(find_mount),
            ..MountHandler::mock()
        };

        // Act
        let result = handler.execute();

        // Assert
        let error = result.expect_err("should be an error");
        let error = error
            .current_context()
            .downcast_ref::<AlreadyMounted>()
            .expect("should be able to downcast");
        assert_eq!(error, &AlreadyMounted);
    }

    #[test]
    fn mount_fails_when_unlock_fails() {
        // Arrange
        let mut unlock = MockUnlock::new();
        unlock
            .expect_unlock()
            .returning(|_, _, _| Err(Report::new(UnlockError)));
        let handler = MountHandler {
            unlock: Arc::new(unlock),
            ..MountHandler::mock()
        };

        // Act
        let result = handler.execute();

        // Assert
        let error = result.expect_err("should be an error");
        let error = error
            .current_context()
            .downcast_ref::<UnlockError>()
            .expect("should be able to downcast");
        assert_eq!(error, &UnlockError);
    }
}
