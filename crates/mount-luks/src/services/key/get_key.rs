//! Adapter for retrieving a key by concatenating all configured sources.

use crate::prelude::*;
use std::fs;

/// Retrieve the composite key from all configured sources.
pub trait GetKey: Send + Sync {
    /// Get the key by concatenating all sources of key material.
    fn get_key(&self) -> Result<String, Report<GetKeyError>>;
}

/// [`GetKey`] adapter that reads from file, TPM, and interactive prompt.
#[derive(FromServices)]
pub struct KeyProvider {
    /// Configuration options.
    options: Arc<Options>,
    /// Adapter for unsealing TPM objects with retry.
    unseal: Arc<Retry<dyn Unseal>>,
    /// Adapter for prompting for a password.
    prompt: Arc<dyn PromptPassword>,
}

impl GetKey for KeyProvider {
    fn get_key(&self) -> Result<String, Report<GetKeyError>> {
        let mut components = Vec::new();
        if let Some(path) = &self.options.key_path {
            trace!(path = %path.display(), "Reading key from file");
            let key = fs::read_to_string(path)
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
                .execute(*handle)
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
                .prompt_password("Enter interactive key component: ")
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

/// Errors returned by [`GetKey::get_key`].
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

    /// [`KeyProvider::get_key`] with only a key file.
    #[test]
    fn get_key_adapter_get_key_file() {
        // Arrange
        let dir = TempDirectory::default()
            .create()
            .expect("should be able to create temp directory");
        let key_path = dir.join("test.key");
        fs::write(&key_path, "file-key").expect("should be able to write key file");
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                key_path: Some(key_path),
                ..Options::fixture_without_key_sources()
            })
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        assert_eq!(output.ok(), Some(String::from("file-key")));
    }

    /// [`KeyProvider::get_key`] with only the TPM.
    #[test]
    fn get_key_adapter_get_tpm() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                tpm_handle: Some(PersistentHandle::fixture()),
                ..Options::fixture_without_key_sources()
            })
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        assert_eq!(output.ok(), Some(String::from("fixture-tpm-key")));
    }

    /// [`KeyProvider::get_key`] with only the prompt.
    #[test]
    fn get_key_adapter_get_prompt() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                key_prompt: Some(true),
                ..Options::fixture_without_key_sources()
            })
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        assert_eq!(output.ok(), Some(String::from("fixture-password")));
    }

    /// [`KeyProvider::get_key`] with a key file, the TPM, and the prompt.
    #[test]
    fn get_key_adapter_get_all_sources() {
        // Arrange
        let dir = TempDirectory::default()
            .create()
            .expect("should be able to create temp directory");
        let key_path = dir.join("test.key");
        fs::write(&key_path, "file").expect("should be able to write key file");
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                key_path: Some(key_path),
                ..Options::fixture()
            })
            .with_instance(UnsealStub {
                key: Ok(String::from("tpm")),
            })
            .with_instance(PromptPasswordStub {
                password: Ok(String::from("prompt")),
            })
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        assert_eq!(output.ok(), Some(String::from("filetpmprompt")));
    }

    /// [`KeyProvider::get_key`] with no key sources.
    #[test]
    fn get_key_adapter_get_no_sources() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options::fixture_without_key_sources())
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        let report = output.expect_err("should not be able to get a key without sources");
        assert_eq!(report.current_context(), &GetKeyError::Required);
    }

    /// [`KeyProvider::get_key`] with an empty key file.
    #[test]
    fn get_key_adapter_get_empty_key_file() {
        // Arrange
        let dir = TempDirectory::default()
            .create()
            .expect("should be able to create temp directory");
        let key_path = dir.join("test.key");
        fs::write(&key_path, "").expect("should be able to write key file");
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                key_path: Some(key_path),
                ..Options::fixture_without_key_sources()
            })
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        assert_eq!(output.ok(), Some(String::new()));
    }

    /// [`KeyProvider::get_key`] with an empty TPM value.
    #[test]
    fn get_key_adapter_get_empty_tpm() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                tpm_handle: Some(PersistentHandle::fixture()),
                ..Options::fixture_without_key_sources()
            })
            .with_instance(UnsealStub {
                key: Ok(String::new()),
            })
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        assert_eq!(output.ok(), Some(String::new()));
    }

    /// [`KeyProvider::get_key`] with a prompt value of only whitespace.
    #[test]
    fn get_key_adapter_get_empty_prompt() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                key_prompt: Some(true),
                ..Options::fixture_without_key_sources()
            })
            .with_instance(PromptPasswordStub {
                password: Ok(String::from("   ")),
            })
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        assert_eq!(output.ok(), Some(String::new()));
    }

    /// [`KeyProvider::get_key`] with a key file that doesn't exist.
    #[test]
    fn get_key_adapter_get_missing_key_file() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                key_path: Some(PathBuf::from("/nonexistent/path/to/missing.key")),
                ..Options::fixture_without_key_sources()
            })
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        let report = output.expect_err("should not be able to read a missing key file");
        assert_eq!(report.current_context(), &GetKeyError::KeyFile);
    }

    /// [`KeyProvider::get_key`] when the TPM can't unseal.
    #[test]
    fn get_key_adapter_get_unseal_error() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                tpm_handle: Some(PersistentHandle::fixture()),
                ..Options::fixture_without_key_sources()
            })
            .with_instance(UnsealStub {
                key: Err(UnsealError::Failed),
            })
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        let report = output.expect_err("should not be able to get a key when unseal fails");
        assert_eq!(report.current_context(), &GetKeyError::Tpm);
    }

    /// [`KeyProvider::get_key`] when the prompt can't be read.
    #[test]
    fn get_key_adapter_get_prompt_error() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options {
                key_prompt: Some(true),
                ..Options::fixture_without_key_sources()
            })
            .with_instance(PromptPasswordStub {
                password: Err(PromptPasswordError::Read),
            })
            .build();
        let provider = services.expect::<KeyProvider>();
        // Act
        let output = provider.get_key();
        // Assert
        let report = output.expect_err("should not be able to get a key when the prompt fails");
        assert_eq!(report.current_context(), &GetKeyError::Prompt);
    }
}
