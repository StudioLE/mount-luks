use crate::prelude::*;

/// Check if the TPM handle is already in use.
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_evictcontrol.1/>
pub fn evict_object(handle: PersistentHandle) -> Result<(), Report<EvictObjectError>> {
    Command::new("tpm2_evictcontrol")
        .arg("--hierarchy")
        .arg(OWNER_HIERARCHY)
        .arg("--object-context")
        .arg(handle.to_string())
        .output()
        .expect("should be able to execute `tpm2_evictcontrol`")
        .ok_or(EvictObjectError)
        .attach_key_value("Handle", &handle.to_string())
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Unable to evict object")]
pub struct EvictObjectError;
