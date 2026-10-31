//! List persistent TPM handles with `tpm2_getcap`.

use crate::prelude::*;

/// Get persistent TPM handles.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_getcap.1/>
pub trait GetHandles: Send + Sync {
    /// List all persistent TPM handles.
    fn get_handles(&self) -> Result<Vec<PersistentHandle>, Report<GetHandlesError>>;
}

impl GetHandles for Tpm2Adapter {
    fn get_handles(&self) -> Result<Vec<PersistentHandle>, Report<GetHandlesError>> {
        let response = Command::new("tpm2_getcap")
            .arg("handles-persistent")
            .output()
            .change_context(GetHandlesError::Run)?
            .to_response();
        if !response.status.success() {
            return Err(Report::new(GetHandlesError::Failed).attach_response(response));
        }
        let stdout = response.output.unwrap_or_default();
        let handles = stdout
            .lines()
            .map(|line| line.trim_start_matches("- "))
            .filter(|line| line.starts_with("0x"))
            .filter_map(|value| PersistentHandle::from_str(value.trim()).ok())
            .collect();
        Ok(handles)
    }
}

/// Errors returned by [`GetHandles::get_handles`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum GetHandlesError {
    /// Unable to start `tpm2_getcap` or exchange data with it.
    #[error("Unable to run `tpm2_getcap`")]
    Run,
    /// `tpm2_getcap` exited with an error.
    #[error("Unable to get persistent TPM handles")]
    Failed,
}
