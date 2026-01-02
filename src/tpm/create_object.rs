use crate::prelude::*;
use rpassword::prompt_password;
use std::process::Stdio;

/// Create a child object.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_create.1/>
pub fn create_object() -> Result<(), Report<CreateObjectError>> {
    let key = prompt_password("Enter the key:").change_context(CreateObjectError::Prompt)?;
    create_object_from_input(&key)
}

pub fn create_object_from_input(key: &str) -> Result<(), Report<CreateObjectError>> {
    Command::new("tpm2_create")
        .arg("--parent-context")
        .arg(TPM_PRIMARY_CONTEXT_PATH.display().to_string())
        .arg("--hash-algorithm")
        .arg(HASH_ALGORITHM)
        .arg("--public")
        .arg(TPM_OBJ_PUBLIC_PATH.display().to_string())
        .arg("--private")
        .arg(TPM_OBJ_PRIVATE_PATH.display().to_string())
        .arg("--policy")
        .arg(TPM_POLICY_PATH.display().to_string())
        .arg("--sealing-input")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("should be able to spawn `tpm2_create`")
        .write_to_stdin(key)
        .wait_with_output()
        .expect("should be able to wait for `tpm2_create`")
        .ok_or(CreateObjectError::Failed)
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum CreateObjectError {
    #[error("Unable to read key from prompt")]
    Prompt,
    #[error("Unable to create object")]
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _create_object_from_input() {
        assert!(is_root().is_ok(), "Root is required to run this test");
        // Arrange
        let input = "Hello, world!";
        create_policy().expect("Should be able to create policy");
        create_primary().expect("Should be able to create primary");

        // Act
        let result = create_object_from_input(input);

        // Preview
        if let Err(report) = &result {
            eprintln!("{report:?}");
        }

        // Assert
        assert!(result.is_ok());
    }
}
