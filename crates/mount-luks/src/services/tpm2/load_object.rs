use crate::prelude::*;

/// Load a TPM object.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_load.1/>
pub trait LoadObject: Send + Sync {
    /// Load both the private and public portions of an object into the TPM.
    fn load_object(&self) -> Result<(), Report<LoadObjectError>>;
}

impl LoadObject for Tpm2Adapter {
    fn load_object(&self) -> Result<(), Report<LoadObjectError>> {
        Command::new("tpm2_load")
            .arg("--parent-context")
            .arg(TPM_PRIMARY_CONTEXT_PATH.as_path())
            .arg("--public")
            .arg(TPM_OBJ_PUBLIC_PATH.as_path())
            .arg("--private")
            .arg(TPM_OBJ_PRIVATE_PATH.as_path())
            .arg("--key-context")
            .arg(TPM_OBJ_CONTEXT_PATH.as_path())
            .output()
            .change_context(LoadObjectError::Run)?
            .ok_or(LoadObjectError::Failed)
    }
}

/// Errors returned by [`LoadObject::load_object`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum LoadObjectError {
    /// Unable to start `tpm2_load` or exchange data with it.
    #[error("Unable to run `tpm2_load`")]
    Run,
    /// `tpm2_load` exited with an error.
    #[error("Unable to load object into the TPM")]
    Failed,
}
