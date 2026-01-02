use crate::prelude::*;
use rpassword::prompt_password;
use std::process::Stdio;

pub fn add_key(options: &Options) -> Result<(), Report<KeyError>> {
    let key = get_key(options)?;
    if check_key(options, &key).is_ok() {
        return Err(Report::new(KeyError::Exists));
    }
    add_key_internal(options, &key)?;
    Ok(())
}

fn add_key_internal(options: &Options, key: &str) -> Result<(), Report<KeyError>> {
    debug!("Key is {} characters", key.len());
    let existing = prompt_password("Enter existing passphrase: ")
        .change_context(KeyError::Add)
        .attach("Failed to read existing passphrase")?;
    let input = [&existing, key, key].join("\n");
    Command::new("cryptsetup")
        .arg("luksAddKey")
        .arg(options.partition_path.display().to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("should be able to spawn `cryptsetup luksAddKey`")
        .write_to_stdin(&input)
        .wait_with_output()
        .expect("should be able to wait on `cryptsetup luksAddKey`")
        .ok_or(KeyError::Add)
}
