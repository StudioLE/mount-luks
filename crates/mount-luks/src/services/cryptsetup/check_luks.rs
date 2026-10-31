use crate::prelude::*;

/// Check if a partition is encrypted with LUKS.
pub trait CheckLuks: Send + Sync {
    /// Check if a partition is LUKS encrypted.
    fn check_luks(&self, partition: &Path) -> Result<(), Report<CheckLuksError>>;
}

impl CheckLuks for CryptsetupAdapter {
    fn check_luks(&self, partition: &Path) -> Result<(), Report<CheckLuksError>> {
        let response = Command::new("cryptsetup")
            .arg("isLuks")
            .arg(partition)
            .output()
            .change_context(CheckLuksError::Run)?
            .to_response();
        if response.status.success() {
            return Ok(());
        }
        if response.output.is_none() && response.error.is_none() {
            return Err(Report::new(CheckLuksError::NotLuks));
        }
        Err(Report::new(CheckLuksError::Unexpected).attach_response(response))
    }
}

/// Errors returned by [`CheckLuks::check_luks`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CheckLuksError {
    /// Unable to start `cryptsetup isLuks` or exchange data with it.
    #[error("Unable to run `cryptsetup isLuks`")]
    Run,
    #[error("Partition is not encrypted with LUKS")]
    NotLuks,
    #[error("Unable to determine if encrypted with LUKS")]
    Unexpected,
}
