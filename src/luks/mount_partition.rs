use crate::prelude::*;

pub fn mount_partition(options: &Options) -> Result<(), Report<MountError>> {
    let mapper_path = &options.get_mapper_path();
    Command::new("mount")
        .arg(mapper_path.display().to_string())
        .arg(options.mount_path.display().to_string())
        .output()
        .expect("should be able to execute `mount`")
        .ok_or(MountError)
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Failed to mount partition")]
pub struct MountError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _mount_partition() {
        // Arrange
        let options = Options::read_options().expect("Should be able to read options");

        // Act
        let result = mount_partition(&options);

        // Assert
        if is_root().is_ok() {
            assert!(result.is_ok());
        } else if let Err(report) = &result {
            eprintln!("{report:?}");
            let _error = report
                .downcast_ref::<MountError>()
                .expect("should be MountError");
        }
    }
}
