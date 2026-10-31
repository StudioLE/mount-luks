//! Create a TPM PCR policy with `tpm2_createpolicy`.

use crate::prelude::*;

/// Create a TPM PCR policy.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_createpolicy.1/>
pub trait CreatePolicy: Send + Sync {
    /// Create a policy that requires the TPM to have a certain PCR value.
    fn create_policy(&self) -> Result<(), Report<CreatePolicyError>>;
}

impl CreatePolicy for Tpm2Adapter {
    fn create_policy(&self) -> Result<(), Report<CreatePolicyError>> {
        Command::new("tpm2_createpolicy")
            .arg("--policy-pcr")
            .arg("--pcr-list")
            .arg(POLICY.as_str())
            .arg("--policy")
            .arg(TPM_POLICY_PATH.as_path())
            .output()
            .change_context(CreatePolicyError::Run)?
            .ok_or(CreatePolicyError::Failed)
    }
}

/// Errors returned by [`CreatePolicy::create_policy`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CreatePolicyError {
    /// Unable to start `tpm2_createpolicy` or exchange data with it.
    #[error("Unable to run `tpm2_createpolicy`")]
    Run,
    /// `tpm2_createpolicy` exited with an error.
    #[error("Unable to create TPM policy")]
    Failed,
}
