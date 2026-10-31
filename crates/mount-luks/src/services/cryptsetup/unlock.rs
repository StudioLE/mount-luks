use crate::prelude::*;
use std::process::Stdio;

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
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("should be able to spawn `cryptsetup luksOpen`")
            .write_to_stdin(key)
            .wait_with_output()
            .expect("should be able to wait on `cryptsetup luksOpen`")
            .ok_or(UnlockError::Failed)
    }
}

/// Errors returned by [`Unlock::unlock`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UnlockError {
    /// `cryptsetup luksOpen` exited with an error.
    #[error("Failed to unlock LUKS partition")]
    Failed,
}
