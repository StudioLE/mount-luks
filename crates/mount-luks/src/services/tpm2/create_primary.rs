//! Create a TPM primary key with `tpm2_createprimary`.

use crate::prelude::*;

/// Create a TPM primary key.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_createprimary.1/>
pub trait CreatePrimary: Send + Sync {
    /// Create a primary key in the TPM owner hierarchy.
    fn create_primary(&self) -> Result<(), Report<CreatePrimaryError>>;
}

impl CreatePrimary for Tpm2Adapter {
    fn create_primary(&self) -> Result<(), Report<CreatePrimaryError>> {
        Command::new("tpm2_createprimary")
            .arg("--hierarchy")
            .arg(OWNER_HIERARCHY)
            .arg("--hash-algorithm")
            .arg(HASH_ALGORITHM)
            .arg("--key-context")
            .arg(TPM_PRIMARY_CONTEXT_PATH.as_path())
            .output()
            .change_context(CreatePrimaryError::Run)?
            .ok_or(CreatePrimaryError::Failed)
    }
}

/// Errors returned by [`CreatePrimary::create_primary`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CreatePrimaryError {
    /// Unable to start `tpm2_createprimary` or exchange data with it.
    #[error("Unable to run `tpm2_createprimary`")]
    Run,
    /// `tpm2_createprimary` exited with an error.
    #[error("Unable to create TPM primary key")]
    Failed,
}
