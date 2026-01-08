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
    /// Example: `/root/.config/mount-luks/e.key`
    pub key_path: Option<PathBuf>,
    /// Optional TPM persistent handle address
    ///
    /// Example: `0x81000000`
    pub tpm_handle: Option<PersistentHandle>,
    /// Optional should an interactive key be required?
    pub key_prompt: Option<bool>,
    /// Hide the UI header
    pub no_header: Option<bool>,
}

impl Options {
    pub fn read_options(config_path: Option<PathBuf>) -> Result<Options, Report<OptionsError>> {
        let path = match config_path {
            Some(path) => path,
            None => get_default_config_path()?,
        };
        trace!(path = %path.display(), "Reading options from path");
        let file = File::open(&path)
            .change_context(OptionsError::Read)
            .attach_path(&path)?;
        serde_yaml::from_reader(file)
            .change_context(OptionsError::Deserialize)
            .attach_path(&path)
    }

    pub fn get_mapper_path(&self) -> PathBuf {
        PathBuf::from("/dev/mapper").join(&self.mapper_name)
    }
}

fn get_default_config_path() -> Result<PathBuf, Report<OptionsError>> {
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
    match paths.len() {
        0 => return Err(Report::new(OptionsError::NoFile)),
        1 => {}
        _ => return Err(Report::new(OptionsError::MultipleFiles)),
    }
    Ok(paths
        .into_iter()
        .next()
        .expect("should be at least one options file"))
}

fn get_paths() -> Result<Vec<PathBuf>, Report<OptionsError>> {
    let dir = config_dir()
        .expect("should be able to get config directory")
        .join(APP_NAME);
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
    #[error("Multiple options files found")]
    MultipleFiles,
    #[error("Unable to read options file")]
    Read,
    #[error("Unable to deserialize options file")]
    Deserialize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;

    #[test]
    fn _read_options() {
        assert!(is_root().is_ok(), "Root is required to run this test");
        // Arrange
        // Act
        let _options = Options::read_options(None).expect("Should be able to read options");

        // Assert
    }

    #[test]
    fn read_options_from_specific_config_file() {
        // Arrange
        let dir = TempDirectory::default()
            .create()
            .expect("should create temp directory");
        let mut paths = Vec::new();
        for i in 1..=3 {
            let path = dir.join(format!("config{i}.yaml"));
            let content = format!(
                "partition_path: /dev/sda{i}\nmapper_name: test-{i}\nmount_path: /mnt/test{i}\n"
            );
            write(&path, content).expect("should write config file");
            paths.push(path);
        }
        let target_path = paths.get(1).expect("should have path at index 1").clone();

        // Act
        let options = Options::read_options(Some(target_path)).expect("should read options");

        // Assert
        assert_eq!(options.mapper_name, "test-2");
        assert_eq!(options.partition_path, PathBuf::from("/dev/sda2"));
        assert_eq!(options.mount_path, PathBuf::from("/mnt/test2"));
    }
}
