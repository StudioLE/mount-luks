use crate::prelude::*;

/// Create a TPM primary key.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_createprimary.1/>
#[cfg_attr(test, mockall::automock)]
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
            .expect("should be able to execute `tpm2_createprimary`")
            .ok_or(CreatePrimaryError)
    }
}

/// Error returned by [`CreatePrimary::create_primary`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Unable to create TPM primary key")]
pub struct CreatePrimaryError;
