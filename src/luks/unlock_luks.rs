use crate::prelude::*;
use std::process::Stdio;

pub fn unlock_luks(options: &Options) -> Result<(), Report<KeyError>> {
    let key = get_key(options)?;
    check_key(options, &key)?;
    unlock_luks_with_key(options, &key)?;
    Ok(())
}

fn unlock_luks_with_key(options: &Options, key: &str) -> Result<(), Report<KeyError>> {
    Command::new("cryptsetup")
        .arg("luksOpen")
        .arg("--key-file=-") // Read password from stdin
        .arg(options.partition_path.display().to_string())
        .arg(&options.mapper_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("should be able to spawn `cryptsetup luksOpen`")
        .write_to_stdin(key)
        .wait_with_output()
        .expect("should be able to wait on `cryptsetup luksOpen`")
        .ok_or(KeyError::Unlock)
}
