use crate::prelude::*;

/// Unseal data from a TPM object.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_unseal.1/>
#[cfg_attr(test, mockall::automock)]
pub trait Unseal: Send + Sync {
    /// Unseal data from a persistent TPM object.
    fn unseal(&self, handle: PersistentHandle) -> Result<String, Report<UnsealError>>;
}

impl Unseal for Tpm2Adapter {
    fn unseal(&self, handle: PersistentHandle) -> Result<String, Report<UnsealError>> {
        let context = handle.to_string();
        let response = Command::new("tpm2_unseal")
            .arg("--object-context")
            .arg(&context)
            .arg("--auth")
            .arg(format!("pcr:{}", &*POLICY))
            .output()
            .expect("should be able to execute `tpm2_unseal`")
            .to_response();
        if !response.status.success() {
            return Err(Report::new(UnsealError).attach_response(response));
        }
        Ok(response.output.unwrap_or_default())
    }
}

/// Error returned by [`Unseal::unseal`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Unable to unseal TPM object")]
pub struct UnsealError;
