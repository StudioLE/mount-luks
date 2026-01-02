use crate::prelude::*;
use dirs::config_dir;
use serde::Deserialize;
use std::fs::{File, read_dir};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Options {
    /// Path of the LUKS partition
    ///
    /// Example: `/dev/nvme0n1p9`
    pub partition_path: PathBuf,
    /// Name to use for the mapper device
    ///
    /// Examples: `e`, `encrypted`, `my-device`
    pub mapper_name: String,
    /// Path to mount the unlocked LUKS partition
    ///
    /// Example: `/mnt/e`
    pub mount_path: PathBuf,
    /// Optional path to a file containing the LUKS key
    ///
    /// Ideally this is stored on an external USB device which is removed when not required
    ///
    /// Example: `/root/.config/mount_luks/e.key`
    pub key_path: Option<PathBuf>,
    /// Optional TPM persistent handle address
    ///
    /// Example: `0x81000000`
    pub tpm_handle: Option<PersistentHandle>,
    /// Optional should an interactive key be required?
    pub key_prompt: Option<bool>,
}

impl Options {
    pub fn read_options() -> Result<Options, Report<OptionsError>> {
        let paths = get_paths()?;
        trace!(
            "Found {} options files:\n{}",
            paths.len(),
            paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );
        if paths.is_empty() {
            return Err(Report::new(OptionsError::NoFile));
        }
        let path = paths.first().expect("should be at least one options file");
        trace!("Reading options from: {}", path.display());
        let file = File::open(path).change_context(OptionsError::Read)?;
        serde_yaml::from_reader(file).change_context(OptionsError::Deserialize)
    }

    pub fn get_mapper_path(&self) -> PathBuf {
        PathBuf::from("/dev/mapper").join(&self.mapper_name)
    }
}

fn get_paths() -> Result<Vec<PathBuf>, Report<OptionsError>> {
    let dir = config_dir()
        .expect("should be able to get config directory")
        .join("mount_luks");
    let paths = read_dir(&dir)
        .change_context(OptionsError::ReadDir)?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            if let Ok(file_type) = entry.file_type()
                && !file_type.is_file()
            {
                return None;
            }
            let path = entry.path();
            match path.extension()?.to_str()? {
                "yaml" | "yml" => {}
                _ => return None,
            }
            Some(path)
        })
        .collect();
    Ok(paths)
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum OptionsError {
    #[error("Unable to read config directory")]
    ReadDir,
    #[error("Options file does not exist")]
    NoFile,
    #[error("Unable to read options file")]
    Read,
    #[error("Unable to deserialize options file")]
    Deserialize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _read_options() {
        assert!(is_root().is_ok(), "Root is required to run this test");
        // Arrange
        // Act
        let _options = Options::read_options().expect("Should be able to read options");

        // Assert
    }
}
