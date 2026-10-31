use crate::prelude::*;

/// Make a TPM object persistent.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_evictcontrol.1/>
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
            .change_context(PersistError::Run)?
            .ok_or(PersistError::Failed)
            .attach("Handle", handle)
    }
}

/// Errors returned by [`Persist::persist`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PersistError {
    /// Unable to start `tpm2_evictcontrol` or exchange data with it.
    #[error("Unable to run `tpm2_evictcontrol`")]
    Run,
    /// `tpm2_evictcontrol` exited with an error.
    #[error("Unable to persist TPM object")]
    Failed,
}
