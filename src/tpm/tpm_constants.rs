use crate::prelude::*;
use std::env::temp_dir;
use std::fs::create_dir;
use std::sync::LazyLock;

#[cfg(test)]
pub static EXAMPLE_HANDLE: LazyLock<PersistentHandle> =
    LazyLock::new(|| PersistentHandle::new(0x8100FFFF).expect("valid handle"));

/// PCR bank algorithm
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/common/alg/>
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/common/pcr/>
const BANK_ALGORITHM: &str = "sha256";

/// PCR register index
///
/// `7`: Secure Boot State. Contains the full contents of PK/KEK/db, as well as the specific
/// certificates used to validate each boot application.
///
/// - <https://wiki.archlinux.org/title/Trusted_Platform_Module#Accessing_PCR_registers>
const PCR_INDEX: u8 = 7;

/// PCR policy
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/common/pcr/>
pub static POLICY: LazyLock<String> = LazyLock::new(|| format!("{BANK_ALGORITHM}:{PCR_INDEX}"));

/// The hierarchy under which the object is created.
///
/// This will also dictate which authorization secret (if any) must be supplied.
///
/// `o`: `TPM_RH_OWNER`
/// `p`: `TPM_RH_PLATFORM`
/// `e`: `TPM_RH_ENDORSEMENT`
/// `n`: `TPM_RH_NULL`
/// `<num>` where a raw number can be used
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_createprimary.1/>
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_evictcontrol.1/>
pub const OWNER_HIERARCHY: &str = "o";

/// The hash algorithm to use for generating the objects name.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_createprimary.1/>
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_create.1/>
pub const HASH_ALGORITHM: &str = "sha256";

/// Get the app specific temp directory.
///
/// The directory is created if it does not already exist.
static TEMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = temp_dir().join(APP_NAME);
    if !dir.exists() {
        create_dir(&dir).expect("should be able to create temp dir");
    }
    dir
});

/// Path of the PCR policy type.
pub static TPM_POLICY_PATH: LazyLock<PathBuf> = LazyLock::new(|| TEMP_DIR.join("policy.dat"));

/// Path of the TPM primary object context.
pub static TPM_PRIMARY_CONTEXT_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| TEMP_DIR.join("primary.ctx"));

/// Path of the public portion of the TPM sealed object.
pub static TPM_OBJ_PUBLIC_PATH: LazyLock<PathBuf> = LazyLock::new(|| TEMP_DIR.join("object.pub"));

/// Path of the sensitive portion of the TPM sealed object.
pub static TPM_OBJ_PRIVATE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| TEMP_DIR.join("object.private"));

/// Path of the TPM sealed object context.
pub static TPM_OBJ_CONTEXT_PATH: LazyLock<PathBuf> = LazyLock::new(|| TEMP_DIR.join("object.ctx"));
