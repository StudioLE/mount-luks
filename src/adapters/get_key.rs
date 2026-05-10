//! Adapter for retrieving a key by concatenating all configured sources.

use crate::prelude::*;
use std::fs::read_to_string;

/// Retrieve the composite key from all configured sources.
#[cfg_attr(test, mockall::automock)]
pub trait GetKey: Send + Sync {
    /// Get the key by concatenating all sources of key material.
    fn get(&self) -> Result<String, Report<GetKeyError>>;
}

/// [`GetKey`] adapter that reads from file, TPM, and interactive prompt.
#[derive(FromServices)]
pub struct GetKeyAdapter {
    /// Configuration options.
    options: Arc<Options>,
    /// Adapter for unsealing TPM objects.
    unseal: Arc<dyn Unseal>,
    /// Adapter for prompting for a password.
    prompt: Arc<dyn PromptPassword>,
}

impl GetKey for GetKeyAdapter {
    fn get(&self) -> Result<String, Report<GetKeyError>> {
        let mut components = Vec::new();
        if let Some(path) = &self.options.key_path {
            trace!(path = %path.display(), "Reading key from file");
            let key = read_to_string(path)
                .change_context(GetKeyError::KeyFile)
                .attach_path(path)?
                .trim()
                .to_owned();
            if key.is_empty() {
                warn!(path = %path.display(), "Key file is empty");
            }
            components.push(key);
        }
        if let Some(handle) = &self.options.tpm_handle {
            trace!(%handle, "Reading key from TPM");
            let key = self
                .unseal
                .unseal(*handle)
                .change_context(GetKeyError::Tpm)
                .attach("Handle", handle)?;
            if key.is_empty() {
                warn!(%handle, "TPM key value is empty");
            }
            components.push(key);
        }
        if self.options.key_prompt == Some(true) {
            trace!("Reading key from prompt");
            let key = self
                .prompt
                .prompt("Enter interactive key component: ")
                .change_context(GetKeyError::Prompt)?
                .trim()
                .to_owned();
            if key.is_empty() {
                warn!("Prompt value is empty");
            }
            components.push(key);
        }
        if components.is_empty() {
            return Err(Report::new(GetKeyError::Required));
        }
        Ok(components.join(""))
    }
}

/// Errors returned by [`GetKey::get`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum GetKeyError {
    /// Unable to read key file.
    #[error("Unable to read key file")]
    KeyFile,
    /// Unable to read key from TPM.
    #[error("Unable to read key from TPM")]
    Tpm,
    /// Unable to read key from prompt.
    #[error("Unable to read key from prompt")]
    Prompt,
    /// At least one key source must be provided.
    #[error("At least one key source must be provided")]
    Required,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn make_adapter(options: Options) -> GetKeyAdapter {
        let unseal = MockUnseal::new();
        let prompt = MockPromptPassword::new();
        GetKeyAdapter {
            options: Arc::new(options),
            unseal: Arc::new(unseal),
            prompt: Arc::new(prompt),
        }
    }

    fn make_adapter_with(
        options: Options,
        unseal: MockUnseal,
        prompt: MockPromptPassword,
    ) -> GetKeyAdapter {
        GetKeyAdapter {
            options: Arc::new(options),
            unseal: Arc::new(unseal),
            prompt: Arc::new(prompt),
        }
    }

    #[test]
    fn get_key_from_file_only() {
        // Arrange
        let dir = TempDirectory::default()
            .create()
            .expect("should create temp directory");
        let key_path = dir.join("test.key");
        fs::write(&key_path, "file-key").expect("should write key file");
        let adapter = make_adapter(Options {
            key_path: Some(key_path),
            ..Options::default()
        });

        // Act
        let result = adapter.get();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.expect("should be ok"), "file-key");
    }

    #[test]
    fn get_key_from_tpm_only() {
        // Arrange
        let handle = PersistentHandle::new(0x81000000).expect("valid handle");
        let mut unseal = MockUnseal::new();
        unseal
            .expect_unseal()
            .returning(|_| Ok(String::from("tpm-key")));
        let adapter = make_adapter_with(
            Options {
                tpm_handle: Some(handle),
                ..Options::default()
            },
            unseal,
            MockPromptPassword::new(),
        );

        // Act
        let result = adapter.get();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.expect("should be ok"), "tpm-key");
    }

    #[test]
    fn get_key_from_prompt_only() {
        // Arrange
        let mut prompt = MockPromptPassword::new();
        prompt
            .expect_prompt()
            .returning(|_| Ok(String::from("prompt-key")));
        let adapter = make_adapter_with(
            Options {
                key_prompt: Some(true),
                ..Options::default()
            },
            MockUnseal::new(),
            prompt,
        );

        // Act
        let result = adapter.get();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.expect("should be ok"), "prompt-key");
    }

    #[test]
    fn get_key_concatenates_all_sources() {
        // Arrange
        let dir = TempDirectory::default()
            .create()
            .expect("should create temp directory");
        let key_path = dir.join("test.key");
        fs::write(&key_path, "file").expect("should write key file");
        let handle = PersistentHandle::new(0x81000000).expect("valid handle");
        let mut unseal = MockUnseal::new();
        unseal
            .expect_unseal()
            .returning(|_| Ok(String::from("tpm")));
        let mut prompt = MockPromptPassword::new();
        prompt
            .expect_prompt()
            .returning(|_| Ok(String::from("prompt")));
        let adapter = make_adapter_with(
            Options {
                key_path: Some(key_path),
                tpm_handle: Some(handle),
                key_prompt: Some(true),
                ..Options::default()
            },
            unseal,
            prompt,
        );

        // Act
        let result = adapter.get();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.expect("should be ok"), "filetpmprompt");
    }

    #[test]
    fn get_key_fails_when_no_sources() {
        // Arrange
        let adapter = make_adapter(Options::default());

        // Act
        let result = adapter.get();

        // Assert
        let report = result.expect_err("should be an error when no key sources are configured");
        assert_eq!(*report.current_context(), GetKeyError::Required);
    }

    #[test]
    fn get_key_with_empty_key_file() {
        // Arrange
        let dir = TempDirectory::default()
            .create()
            .expect("should create temp directory");
        let key_path = dir.join("test.key");
        fs::write(&key_path, "").expect("should write key file");
        let adapter = make_adapter(Options {
            key_path: Some(key_path),
            ..Options::default()
        });

        // Act
        let result = adapter.get();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.expect("should be ok"), "");
    }

    #[test]
    fn get_key_with_empty_tpm_value() {
        // Arrange
        let handle = PersistentHandle::new(0x81000000).expect("valid handle");
        let mut unseal = MockUnseal::new();
        unseal.expect_unseal().returning(|_| Ok(String::new()));
        let adapter = make_adapter_with(
            Options {
                tpm_handle: Some(handle),
                ..Options::default()
            },
            unseal,
            MockPromptPassword::new(),
        );

        // Act
        let result = adapter.get();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.expect("should be ok"), "");
    }

    #[test]
    fn get_key_with_empty_prompt_value() {
        // Arrange
        let mut prompt = MockPromptPassword::new();
        prompt
            .expect_prompt()
            .returning(|_| Ok(String::from("   ")));
        let adapter = make_adapter_with(
            Options {
                key_prompt: Some(true),
                ..Options::default()
            },
            MockUnseal::new(),
            prompt,
        );

        // Act
        let result = adapter.get();

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.expect("should be ok"), "");
    }

    #[test]
    fn get_key_fails_when_key_file_missing() {
        // Arrange
        let adapter = make_adapter(Options {
            key_path: Some(PathBuf::from("/nonexistent/path/to/missing.key")),
            ..Options::default()
        });

        // Act
        let result = adapter.get();

        // Assert
        let report = result.expect_err("should be an error when key file is missing");
        assert_eq!(*report.current_context(), GetKeyError::KeyFile);
    }

    #[test]
    fn get_key_fails_when_tpm_unseal_fails() {
        // Arrange
        let handle = PersistentHandle::new(0x81000000).expect("valid handle");
        let mut unseal = MockUnseal::new();
        unseal
            .expect_unseal()
            .returning(|_| Err(Report::new(UnsealError)));
        let adapter = make_adapter_with(
            Options {
                tpm_handle: Some(handle),
                ..Options::default()
            },
            unseal,
            MockPromptPassword::new(),
        );

        // Act
        let result = adapter.get();

        // Assert
        let report = result.expect_err("should be an error when TPM unseal fails");
        assert_eq!(*report.current_context(), GetKeyError::Tpm);
    }

    #[test]
    fn get_key_fails_when_prompt_fails() {
        // Arrange
        let mut prompt = MockPromptPassword::new();
        prompt
            .expect_prompt()
            .returning(|_| Err(Report::new(PromptPasswordError)));
        let adapter = make_adapter_with(
            Options {
                key_prompt: Some(true),
                ..Options::default()
            },
            MockUnseal::new(),
            prompt,
        );

        // Act
        let result = adapter.get();

        // Assert
        let report = result.expect_err("should be an error when prompt fails");
        assert_eq!(*report.current_context(), GetKeyError::Prompt);
    }
}
