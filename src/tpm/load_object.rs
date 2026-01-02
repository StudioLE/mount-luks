use crate::prelude::*;

/// Load both the private and public portions of an object into the TPM.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_load.1/>
pub fn load_object() -> Result<(), Report<LoadError>> {
    Command::new("tpm2_load")
        .arg("--parent-context")
        .arg(TPM_PRIMARY_CONTEXT_PATH.display().to_string())
        .arg("--public")
        .arg(TPM_OBJ_PUBLIC_PATH.display().to_string())
        .arg("--private")
        .arg(TPM_OBJ_PRIVATE_PATH.display().to_string())
        .arg("--key-context")
        .arg(TPM_OBJ_CONTEXT_PATH.display().to_string())
        .output()
        .expect("should be able to execute `tpm2_load`")
        .ok_or(LoadError)
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Unable to load object into the TPM")]
pub struct LoadError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial_test::serial]
    fn _load_object() {
        assert!(is_root().is_ok(), "Root is required to run this test");
        // Arrange
        let input = "Hello, world!";
        create_policy().expect("Should be able to create policy");
        create_primary().expect("Should be able to create primary");
        create_object_from_input(input).expect("Should be able to create object");

        // Act
        let result = load_object();

        // Preview
        if let Err(report) = &result {
            eprintln!("{report:?}");
        }

        // Assert
        assert!(result.is_ok());
        let value = unseal_object_from_context_path().expect("Should be able to unseal object");
        assert_eq!(value, input);
    }
}
