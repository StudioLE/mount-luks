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
            .arg(format!("pcr:{}", *POLICY))
            .output()
            .expect("should be able to execute `tpm2_unseal`")
            .to_response();
        if !response.status.success() {
            return prepare_report(response);
        }
        Ok(response.output.unwrap_or_default())
    }
}

impl Attempt for dyn Unseal {
    type Input = PersistentHandle;
    type Output = String;
    type Error = UnsealError;

    fn attempt(&self, handle: PersistentHandle) -> Result<String, Report<UnsealError>> {
        self.unseal(handle)
    }

    fn is_retryable(&self, report: &Report<UnsealError>) -> bool {
        *report.current_context() == UnsealError::PcrChanged
    }
}

/// Build the error for a failed `tpm2_unseal` run.
///
/// - Uses [`UnsealError::PcrChanged`] if the code is `0x128`, else [`UnsealError::Failed`]
/// - Attaches stderr, stdout, and exit status from the [`Response`]
/// - Attaches the parsed `Esys_Unseal` code if present
fn prepare_report(response: Response) -> Result<String, Report<UnsealError>> {
    const PCR_CHANGED: &str = "0x128";
    let code = response.error.as_deref().and_then(parse_code);
    let context = if code.as_deref() == Some(PCR_CHANGED) {
        UnsealError::PcrChanged
    } else {
        UnsealError::Failed
    };
    let mut report = Report::new(context).attach_response(response);
    if let Some(code) = code {
        report = report.attach("code", code);
    }
    Err(report)
}

/// Parse the `Esys_Unseal` return code from `tpm2_unseal` stderr.
///
/// - Reads only the `ERROR: Esys_Unseal(<code>)` line; earlier lines also contain `Esys_Unseal(`
fn parse_code(stderr: &str) -> Option<String> {
    stderr.lines().find_map(|line| {
        line.strip_prefix("ERROR: Esys_Unseal(")?
            .split_once(')')
            .map(|(code, _)| code.to_owned())
    })
}

/// Errors returned by [`Unseal::unseal`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UnsealError {
    /// A PCR changed between the policy check and the unseal.
    #[error("PCR changed during unseal")]
    PcrChanged,
    /// Any other `tpm2_unseal` failure.
    #[error("Unable to unseal TPM object")]
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::ExitStatus;

    const PCR_CHANGED_STDERR: &str =
        "WARNING:esys:src/tss2-esys/api/Esys_Unseal.c:295:Esys_Unseal_Finish() Received TPM Error
ERROR:esys:src/tss2-esys/api/Esys_Unseal.c:98:Esys_Unseal() Esys Finish ErrorCode (0x00000128)
ERROR: Esys_Unseal(0x128) - tpm:error(2.0): PCR have changed since checked
ERROR: Unable to run tpm2_unseal";

    const POLICY_FAILED_STDERR: &str =
        "WARNING:esys:src/tss2-esys/api/Esys_Unseal.c:295:Esys_Unseal_Finish() Received TPM Error
ERROR:esys:src/tss2-esys/api/Esys_Unseal.c:98:Esys_Unseal() Esys Finish ErrorCode (0x0000099d)
ERROR: Esys_Unseal(0x99D) - tpm:session(1):a policy check failed
ERROR: Unable to run tpm2_unseal";

    #[test]
    fn parse_code_0x128() {
        assert_eq!(parse_code(PCR_CHANGED_STDERR).as_deref(), Some("0x128"));
    }

    #[test]
    fn parse_code_missing() {
        assert_eq!(parse_code("ERROR: Unable to run tpm2_unseal"), None);
    }

    #[test]
    fn prepare_report_pcr_changed() {
        // Arrange
        let response = failed_response(PCR_CHANGED_STDERR);

        // Act
        let output = prepare_report(response);

        // Assert
        let report = output.expect_err("should fail");
        assert_eq!(*report.current_context(), UnsealError::PcrChanged);
    }

    #[test]
    fn prepare_report_failed() {
        // Arrange
        let response = failed_response(POLICY_FAILED_STDERR);

        // Act
        let output = prepare_report(response);

        // Assert
        let report = output.expect_err("should fail");
        assert_eq!(*report.current_context(), UnsealError::Failed);
    }

    #[test]
    fn unseal_is_retryable() {
        // Arrange
        let unseal: Arc<dyn Unseal> = Arc::new(MockUnseal::new());

        // Act
        let pcr_changed = unseal.is_retryable(&Report::new(UnsealError::PcrChanged));
        let failed = unseal.is_retryable(&Report::new(UnsealError::Failed));

        // Assert
        assert!(pcr_changed);
        assert!(!failed);
    }

    fn failed_response(stderr: &str) -> Response {
        Response {
            error: Some(stderr.to_owned()),
            output: None,
            status: ExitStatus::from_raw(1 << 8),
        }
    }
}
