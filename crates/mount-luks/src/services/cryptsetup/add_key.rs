//! Add a key to a LUKS partition with `cryptsetup luksAddKey`.

use crate::prelude::*;

/// Add a key to a LUKS partition.
pub trait AddKey: Send + Sync {
    /// Add a new key to a LUKS partition using the existing passphrase.
    fn add_key(
        &self,
        partition: &Path,
        existing_key: &str,
        new_key: &str,
    ) -> Result<(), Report<AddKeyError>>;
}

impl AddKey for CryptsetupAdapter {
    fn add_key(
        &self,
        partition: &Path,
        existing_key: &str,
        new_key: &str,
    ) -> Result<(), Report<AddKeyError>> {
        let input = [existing_key, new_key, new_key].join("\n");
        Command::new("cryptsetup")
            .arg("luksAddKey")
            .arg(partition)
            .output_with_input(&input)
            .change_context(AddKeyError::Run)?
            .ok_or(AddKeyError::Failed)
    }
}

/// Errors returned by [`AddKey::add_key`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum AddKeyError {
    /// Unable to start `cryptsetup luksAddKey` or exchange data with it.
    #[error("Unable to run `cryptsetup luksAddKey`")]
    Run,
    /// `cryptsetup luksAddKey` exited with an error.
    #[error("Failed to add LUKS key")]
    Failed,
}
