use crate::prelude::*;

/// Create a policy that requires the TPM to have a certain PCR value.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_createpolicy.1/>
pub fn create_policy() -> Result<(), Report<CreatePolicyError>> {
    Command::new("tpm2_createpolicy")
        .arg("--policy-pcr")
        .arg("--pcr-list")
        .arg(POLICY.to_owned())
        .arg("--policy")
        .arg(TPM_POLICY_PATH.display().to_string())
        .output()
        .expect("should be able to execute `tpm2_createpolicy`")
        .ok_or(CreatePolicyError)
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Unable to create TPM policy")]
pub struct CreatePolicyError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial_test::serial]
    fn _create_policy() {
        assert!(is_root().is_ok(), "Root is required to run this test");
        // Arrange
        // Act
        let result = create_policy();

        // Preview
        if let Err(report) = &result {
            eprintln!("{report:?}");
        }

        // Assert
        assert!(result.is_ok());
    }
}
