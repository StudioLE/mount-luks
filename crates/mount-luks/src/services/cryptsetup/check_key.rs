use crate::prelude::*;
use std::process::Stdio;

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
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("should be able to spawn `cryptsetup luksOpen --test-passphrase`")
            .write_to_stdin(key)
            .wait_with_output()
            .expect("should be able to wait on `cryptsetup luksOpen --test-passphrase`")
            .ok_or(CheckKeyError::Incorrect)
    }
}

/// Errors returned by [`CheckKey::check_key`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CheckKeyError {
    /// The key doesn't unlock the partition.
    #[error("Key is incorrect")]
    Incorrect,
}
