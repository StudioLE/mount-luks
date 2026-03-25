use crate::prelude::*;

/// Check if a partition is encrypted with LUKS.
#[cfg_attr(test, mockall::automock)]
pub trait IsLuks: Send + Sync {
    /// Check if a partition is LUKS encrypted.
    fn is_luks(&self, partition: &Path) -> Result<(), Report<IsLuksError>>;
}

impl IsLuks for CryptsetupAdapter {
    fn is_luks(&self, partition: &Path) -> Result<(), Report<IsLuksError>> {
        let response = Command::new("cryptsetup")
            .arg("isLuks")
            .arg(partition)
            .output()
            .expect("should be able to execute `cryptsetup isLuks`")
            .to_response();
        if response.status.success() {
            return Ok(());
        }
        if response.output.is_none() && response.error.is_none() {
            return Err(Report::new(IsLuksError::NotLuks));
        }
        Err(Report::new(IsLuksError::Unexpected).attach_response(response))
    }
}

/// Errors returned by [`IsLuks::is_luks`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum IsLuksError {
    #[error("Partition is not encrypted with LUKS")]
    NotLuks,
    #[error("Unable to determine if encrypted with LUKS")]
    Unexpected,
}
