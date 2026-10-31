use crate::prelude::*;

/// Unlock a LUKS partition.
pub trait Unlock: Send + Sync {
    /// Unlock a LUKS partition with the given key.
    fn unlock(&self, partition: &Path, mapper: &str, key: &str) -> Result<(), Report<UnlockError>>;
}

impl Unlock for CryptsetupAdapter {
    fn unlock(&self, partition: &Path, mapper: &str, key: &str) -> Result<(), Report<UnlockError>> {
        Command::new("cryptsetup")
            .arg("luksOpen")
            .arg("--key-file=-")
            .arg(partition)
            .arg(mapper)
            .output_with_input(key)
            .change_context(UnlockError::Run)?
            .ok_or(UnlockError::Failed)
    }
}

/// Errors returned by [`Unlock::unlock`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UnlockError {
    /// Unable to start `cryptsetup luksOpen` or exchange data with it.
    #[error("Unable to run `cryptsetup luksOpen`")]
    Run,
    /// `cryptsetup luksOpen` exited with an error.
    #[error("Failed to unlock LUKS partition")]
    Failed,
}
