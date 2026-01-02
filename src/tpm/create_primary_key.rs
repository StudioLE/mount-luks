use crate::prelude::*;

/// The algorithm type for the generated primary key.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_createprimary.1/>
const KEY_ALGORITHM: &str = "ecc";

/// Create and load a primary object under the owner hierarchy.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_createprimary.1/>
pub fn create_primary() -> Result<(), Report<CreatePrimaryError>> {
    Command::new("tpm2_createprimary")
        .arg("--hierarchy")
        .arg(OWNER_HIERARCHY)
        .arg("--hash-algorithm")
        .arg(HASH_ALGORITHM) // Name algorithm
        .arg("--key-algorithm")
        .arg(KEY_ALGORITHM) // Elliptic curve key
        .arg("--key-context")
        .arg(TPM_PRIMARY_CONTEXT_PATH.display().to_string())
        .output()
        .expect("should be able to execute `tpm2_createprimary`")
        .ok_or(CreatePrimaryError)
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Unable to create TPM primary key")]
pub struct CreatePrimaryError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial_test::serial]
    fn _create_primary() {
        assert!(is_root().is_ok(), "Root is required to run this test");
        // Arrange
        // Act
        let result = create_primary();

        // Preview
        if let Err(report) = &result {
            eprintln!("{report:?}");
        }

        // Assert
        assert!(result.is_ok());
    }
}
