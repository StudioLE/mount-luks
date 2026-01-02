use crate::prelude::*;
use rpassword::prompt_password;
use std::fs::read_to_string;

/// Get the key by concatenating all sources of key material.
/// - File
/// - TPM
/// - Prompt
pub fn get_key(options: &Options) -> Result<String, Report<KeyError>> {
    let mut components = Vec::new();
    if let Some(path) = &options.key_path {
        debug!("Reading key from file: {}", path.display());
        let key = read_to_string(path)
            .change_context(KeyError::KeyFile)
            .attach_path(path)?;
        if key.is_empty() {
            warn!("Key file is empty");
        }
        components.push(key);
    }
    if let Some(handle) = &options.tpm_handle {
        debug!("Reading key from TPM: {handle}");
        let key = unseal_persistent_object(handle)
            .change_context(KeyError::Tpm)
            .attach_key_value("Handle", &handle.to_string())?;
        if key.is_empty() {
            warn!("TPM key value is empty");
        }
        components.push(key);
    }
    if options.key_prompt == Some(true) {
        debug!("Reading key from prompt");
        let key = prompt_password("Enter interactive key component: ")
            .change_context(KeyError::Prompt)?;
        if key.is_empty() {
            warn!("Prompt value is empty");
        }
        components.push(key);
    }
    if components.is_empty() {
        return Err(Report::new(KeyError::Required));
    }
    Ok(components.join(""))
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum KeyError {
    #[error("Unable to read key file")]
    KeyFile,
    #[error("Unable to read key from TPM")]
    Tpm,
    #[error("Unable to read key from prompt")]
    Prompt,
    #[error("At least one key source must be provided")]
    Required,
    #[error("Key is incorrect")]
    InvalidKey,
    #[error("Failed to unlock LUKS partition")]
    Unlock,
    #[error("Key already exists")]
    Exists,
    #[error("Failed to add LUKS key")]
    Add,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _get_key() {
        // Arrange
        let options = Options::read_options().expect("Should be able to read options");

        // Act
        let result = get_key(&options);

        // Assert
        assert!(result.is_ok());
    }
}
