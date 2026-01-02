use crate::prelude::*;

/// Unseal a peristent TPM object from its handle.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_unseal.1/>
pub fn unseal_persistent_object(handle: &PersistentHandle) -> Result<String, Report<UnsealError>> {
    let context = handle.to_string();
    unseal_object(&context)
}

/// Unseal a TPM object from its context path.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_unseal.1/>
#[cfg(test)]
pub(crate) fn unseal_object_from_context_path() -> Result<String, Report<UnsealError>> {
    let context = TPM_OBJ_CONTEXT_PATH.display().to_string();
    unseal_object(&context)
}

/// Unseal a TPM object from its context.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_unseal.1/>
pub fn unseal_object(context: &str) -> Result<String, Report<UnsealError>> {
    let response = Command::new("tpm2_unseal")
        .arg("--object-context")
        .arg(context)
        .arg("--auth")
        .arg(format!("pcr:{}", POLICY.to_owned()))
        .output()
        .expect("should be able to execute `tpm2_unseal`")
        .to_response();
    if !response.status.success() {
        return Err(Report::new(UnsealError).attach_response(response));
    }
    let stdout = response.output.unwrap_or_default();
    Ok(stdout)
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Unable to unseal TPM object")]
pub struct UnsealError;
