use crate::prelude::*;

pub fn check_partition_exist(options: &Options) -> Result<(), Report<NoPartition>> {
    if options.partition_path.exists() {
        Ok(())
    } else {
        Err(Report::new(NoPartition).attach_path(&options.partition_path))
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Parition does not exist")]
pub struct NoPartition;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _check_partition_exist() {
        // Arrange
        let options = Options::read_options().expect("Should be able to read options");

        // Act
        let result = check_partition_exist(&options);

        // Assert
        if let Err(report) = &result {
            eprintln!("{report:?}");
        }
        assert!(result.is_ok());
    }
}
