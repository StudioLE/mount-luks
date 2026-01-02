use crate::prelude::*;

/// Check if the TPM handle is already in use.
pub fn check_handle(options: &Options) -> Result<(), Report<CheckHandleError>> {
    let target_handle = options.tpm_handle.unwrap_or_default();
    let handles = get_handles()?;
    if handles.contains(&target_handle) {
        let report = Report::new(CheckHandleError::HandleInUse)
            .attach_key_value("Handle", &target_handle.to_string());
        Err(report)
    } else {
        Ok(())
    }
}

///  Get handles of persistent objects.
///
/// - <https://tpm2-tools.readthedocs.io/en/latest/man/tpm2_getcap.1/>
pub fn get_handles() -> Result<Vec<PersistentHandle>, Report<CheckHandleError>> {
    let response = Command::new("tpm2_getcap")
        .arg("handles-persistent")
        .output()
        .expect("should be able to execute `tpm2_getcap`")
        .to_response();
    if !response.status.success() {
        return Err(Report::new(CheckHandleError::Failed).attach_response(response));
    }
    let stdout = response.output.unwrap_or_default();
    let handles = stdout
        .lines()
        .map(|line| line.trim_start_matches("- "))
        .filter(|line| line.starts_with("0x"))
        .filter_map(|value| PersistentHandle::from_str(value.trim()).ok())
        .collect();
    Ok(handles)
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum CheckHandleError {
    #[error("Unable to check persistent TPM handles")]
    Failed,
    #[error("TPM handle is already in use")]
    HandleInUse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial_test::serial]
    fn _get_handles() {
        assert!(is_root().is_ok(), "Root is required to run this test");
        // Arrange
        // Act
        let result = get_handles();

        // Assert
        assert!(result.is_ok());
        if let Ok(handles) = result {
            eprintln!(
                "Persistent handles:\n{}",
                handles
                    .iter()
                    .map(|a| a.to_string().clone())
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }

    #[test]
    #[serial_test::serial]
    fn _check_handle() {
        assert!(is_root().is_ok(), "Root is required to run this test");

        // Arrange
        let options = Options {
            tpm_handle: Some(EXAMPLE_HANDLE.to_owned()),
            ..Options::default()
        };

        // Act
        let result = check_handle(&options);

        // Preview
        if let Err(report) = &result {
            eprintln!("{report:?}");
        }

        // Assert
        assert!(result.is_ok());
    }
}
