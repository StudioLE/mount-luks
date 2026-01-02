use crate::prelude::*;

pub fn is_luks_partition(options: &Options) -> Result<(), Report<IsLuksError>> {
    let response = Command::new("cryptsetup")
        .arg("isLuks")
        .arg(options.partition_path.display().to_string())
        .output()
        .expect("should be able to execute `cryptsetup isLuks`")
        .to_response();
    if response.status.success() {
        return Ok(());
    }
    if response.output.is_none() && response.error.is_none() {
        bail!(IsLuksError::NotLuks);
    }
    Err(Report::new(IsLuksError::Unexpected).attach_response(response))
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum IsLuksError {
    #[error("Parition is not encrypted with LUKS")]
    NotLuks,
    #[error("Unable to determine if encrypted with LUKS")]
    Unexpected,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _is_luks_partition() {
        assert!(is_root().is_ok(), "Root is required to run this test");

        // Arrange
        let options = Options::read_options().expect("Should be able to read options");

        // Act
        let result = is_luks_partition(&options);

        // Assert
        if let Err(report) = &result {
            eprintln!("{report:?}");
        }
        assert!(result.is_ok());
    }
}
