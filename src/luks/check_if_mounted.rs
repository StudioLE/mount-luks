use crate::prelude::*;

pub fn check_if_mounted(options: &Options) -> Result<(), Report<AlreadyMounted>> {
    let response = Command::new("findmnt")
        .arg("--noheadings")
        .arg(options.mount_path.display().to_string())
        .output()
        .expect("should be able to execute `findmnt`")
        .to_response();
    if response.status.success() {
        Err(Report::new(AlreadyMounted).attach_path(&options.mount_path))
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Partition is already mounted")]
pub struct AlreadyMounted;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _check_if_mounted() {
        // Arrange
        let options = Options::read_options(None).expect("Should be able to read options");

        // Act
        let result = check_if_mounted(&options);

        // Assert
        if let Err(report) = result {
            eprintln!("{report:?}");
            let _error = report
                .downcast_ref::<AlreadyMounted>()
                .expect("should be AlreadyMounted");
        }
    }
}
