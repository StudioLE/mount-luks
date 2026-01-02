use crate::prelude::*;

/// Make a transient object persistent.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_evictcontrol.1/>
pub fn persist_object(options: &Options) -> Result<(), Report<PersistObjectError>> {
    let handle = options.tpm_handle.ok_or(PersistObjectError::Required)?;
    Command::new("tpm2_evictcontrol")
        .arg("--hierarchy")
        .arg(OWNER_HIERARCHY) // Owner hierarchy
        .arg("--object-context")
        .arg(TPM_OBJ_CONTEXT_PATH.display().to_string())
        .arg(handle.to_string())
        .output()
        .expect("should be able to execute `tpm2_evictcontrol`")
        .ok_or(PersistObjectError::Failed)
        .attach_key_value("Handle", &handle.to_string())
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum PersistObjectError {
    #[error("The `tpm_handle` option is required")]
    Required,
    #[error("Unable to persist object")]
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial_test::serial]
    fn _persist_object() {
        assert!(is_root().is_ok(), "Root is required to run this test");

        // Arrange
        let handles = get_handles().expect("Should be able to get handles");
        let handle = next_handle(handles).expect("Should be able to get a handle");
        eprintln!("Using handle: {handle}");
        let options = Options {
            tpm_handle: Some(handle),
            ..Options::default()
        };

        // Act
        let result = persist_object(&options);
        evict_object(handle).expect("Should be able to evict object");

        // Preview
        if let Err(report) = &result {
            eprintln!("{report:?}");
        }

        // Assert
        assert!(result.is_ok());
    }
}
