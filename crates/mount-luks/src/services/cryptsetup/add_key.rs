use crate::prelude::*;
use std::process::Stdio;

/// Add a key to a LUKS partition.
#[cfg_attr(test, mockall::automock)]
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
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("should be able to spawn `cryptsetup luksAddKey`")
            .write_to_stdin(&input)
            .wait_with_output()
            .expect("should be able to wait on `cryptsetup luksAddKey`")
            .ok_or(AddKeyError)
    }
}

/// Error returned by [`AddKey::add_key`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Failed to add LUKS key")]
pub struct AddKeyError;
