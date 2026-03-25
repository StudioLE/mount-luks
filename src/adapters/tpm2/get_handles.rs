use crate::prelude::*;

/// Get persistent TPM handles.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_getcap.1/>
#[cfg_attr(test, mockall::automock)]
pub trait GetHandles: Send + Sync {
    /// List all persistent TPM handles.
    fn get_handles(&self) -> Result<Vec<PersistentHandle>, Report<GetHandlesError>>;
}

impl GetHandles for Tpm2Adapter {
    fn get_handles(&self) -> Result<Vec<PersistentHandle>, Report<GetHandlesError>> {
        let response = Command::new("tpm2_getcap")
            .arg("handles-persistent")
            .output()
            .expect("should be able to execute `tpm2_getcap`")
            .to_response();
        if !response.status.success() {
            return Err(Report::new(GetHandlesError).attach_response(response));
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

/// Error returned by [`GetHandles::get_handles`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Unable to get persistent TPM handles")]
pub struct GetHandlesError;
