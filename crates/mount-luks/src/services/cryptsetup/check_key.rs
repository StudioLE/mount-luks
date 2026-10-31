use crate::prelude::*;

/// Validate a key against a LUKS partition.
pub trait CheckKey: Send + Sync {
    /// Check if a key is valid for a LUKS partition.
    fn check_key(&self, partition: &Path, key: &str) -> Result<(), Report<CheckKeyError>>;
}

impl CheckKey for CryptsetupAdapter {
    fn check_key(&self, partition: &Path, key: &str) -> Result<(), Report<CheckKeyError>> {
        Command::new("cryptsetup")
            .arg("luksOpen")
            .arg("--test-passphrase")
            .arg("--key-file=-")
            .arg(partition)
            .output_with_input(key)
            .change_context(CheckKeyError::Run)?
            .ok_or(CheckKeyError::Incorrect)
    }
}

/// Errors returned by [`CheckKey::check_key`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CheckKeyError {
    /// Unable to start `cryptsetup luksOpen --test-passphrase` or exchange data with it.
    #[error("Unable to run `cryptsetup luksOpen --test-passphrase`")]
    Run,
    /// The key doesn't unlock the partition.
    #[error("Key is incorrect")]
    Incorrect,
}
