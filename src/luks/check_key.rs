use crate::luks::KeyError;
use crate::prelude::*;
use error_stack::Report;
use std::process::{Command, Stdio};

pub fn check_key(options: &Options, key: &str) -> Result<(), Report<KeyError>> {
    Command::new("cryptsetup")
        .arg("luksOpen")
        .arg("--test-passphrase")
        .arg("--key-file=-")
        .arg(options.partition_path.display().to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("should be able to spawn `cryptsetup luksOpen --test-passphrase`")
        .write_to_stdin(key)
        .wait_with_output()
        .expect("should be able to wait on `cryptsetup luksOpen --test-passphrase`")
        .ok_or(KeyError::InvalidKey)
}
