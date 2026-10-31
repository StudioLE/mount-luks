use crate::prelude::*;
use std::process::Stdio;

/// Create a sealed TPM object.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_create.1/>
#[cfg_attr(test, mockall::automock)]
pub trait CreateObject: Send + Sync {
    /// Seal input data into a TPM object.
    fn create_object(&self, input: &str) -> Result<(), Report<CreateObjectError>>;
}

impl CreateObject for Tpm2Adapter {
    fn create_object(&self, input: &str) -> Result<(), Report<CreateObjectError>> {
        Command::new("tpm2_create")
            .arg("--parent-context")
            .arg(TPM_PRIMARY_CONTEXT_PATH.as_path())
            .arg("--hash-algorithm")
            .arg(HASH_ALGORITHM)
            .arg("--public")
            .arg(TPM_OBJ_PUBLIC_PATH.as_path())
            .arg("--private")
            .arg(TPM_OBJ_PRIVATE_PATH.as_path())
            .arg("--policy")
            .arg(TPM_POLICY_PATH.as_path())
            .arg("--sealing-input")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("should be able to spawn `tpm2_create`")
            .write_to_stdin(input)
            .wait_with_output()
            .expect("should be able to wait for `tpm2_create`")
            .ok_or(CreateObjectError)
    }
}

/// Error returned by [`CreateObject::create_object`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Unable to create TPM object")]
pub struct CreateObjectError;
