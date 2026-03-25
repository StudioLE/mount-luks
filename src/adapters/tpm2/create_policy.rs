use crate::prelude::*;

/// Create a TPM PCR policy.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_createpolicy.1/>
#[cfg_attr(test, mockall::automock)]
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
            .expect("should be able to execute `tpm2_createpolicy`")
            .ok_or(CreatePolicyError)
    }
}

/// Error returned by [`CreatePolicy::create_policy`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Unable to create TPM policy")]
pub struct CreatePolicyError;
