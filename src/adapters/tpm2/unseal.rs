use crate::prelude::*;

/// `TPM_RC_PCR_CHANGED`: a PCR changed between the policy check and the unseal.
const PCR_CHANGED: &str = "0x128";

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
            return prepare_report(response);
        }
        Ok(response.output.unwrap_or_default())
    }
}

fn prepare_report(response: Response) -> Result<String, Report<UnsealError>> {
    let code = response
        .error
        .as_deref()
        .and_then(|stderr| stderr.split_once("Esys_Unseal(")?.1.split_once(')'))
        .map(|(code, _)| code.to_owned());
    let mut report = Report::new(UnsealError).attach_response(response);
    if let Some(code) = &code {
        report = report.attach("code", code);
    }
    if code.as_deref() == Some(PCR_CHANGED) {
        report = report.attach("hint", "A PCR changed during the unseal, retry");
    }
    Err(report)
}

/// Error returned by [`Unseal::unseal`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Unable to unseal TPM object")]
pub struct UnsealError;
