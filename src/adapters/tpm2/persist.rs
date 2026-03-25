use crate::prelude::*;

/// Make a TPM object persistent.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_evictcontrol.1/>
#[cfg_attr(test, mockall::automock)]
pub trait Persist: Send + Sync {
    /// Make a transient TPM object persistent at the given handle.
    fn persist(&self, handle: PersistentHandle) -> Result<(), Report<PersistError>>;
}

impl Persist for Tpm2Adapter {
    fn persist(&self, handle: PersistentHandle) -> Result<(), Report<PersistError>> {
        Command::new("tpm2_evictcontrol")
            .arg("--hierarchy")
            .arg(OWNER_HIERARCHY)
            .arg("--object-context")
            .arg(TPM_OBJ_CONTEXT_PATH.as_path())
            .arg(handle.to_string())
            .output()
            .expect("should be able to execute `tpm2_evictcontrol`")
            .ok_or(PersistError)
            .attach("Handle", handle)
    }
}

/// Error returned by [`Persist::persist`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Unable to persist TPM object")]
pub struct PersistError;
