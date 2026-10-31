use crate::prelude::*;
use serde::Deserialize;
use std::fs::{self, File};

/// LUKS partition configuration loaded from YAML.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Options {
    /// GPT PARTUUID of the LUKS partition
    ///
    /// Example: `a1479a4b-5074-4454-8161-d4ba58f6e4d3`
    pub partuuid: String,
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
    /// Require an interactive key prompt
    pub key_prompt: Option<bool>,
    /// Suppress the header output
    pub no_header: Option<bool>,
}

impl Options {
    fn from_file(path: &Path) -> Result<Options, Report<OptionsError>> {
        trace!(path = %path.display(), "Reading options from path");
        let file = File::open(path)
            .change_context(OptionsError::Read)
            .attach_path(path)?;
        serde_yaml::from_reader(file)
            .change_context(OptionsError::Deserialize)
            .attach_path(path)
    }

    /// Path of the mapper device derived from `mapper_name`.
    pub fn get_mapper_path(&self) -> PathBuf {
        PathBuf::from("/dev/mapper").join(&self.mapper_name)
    }

    /// Create an [`Options`] fixture for testing.
    ///
    /// - Enables the TPM and prompt key sources, but not the key file
    #[cfg(test)]
    pub fn fixture() -> Self {
        Self {
            partuuid: String::from("fixture-partuuid"),
            mapper_name: String::from("fixture"),
            mount_path: PathBuf::from("/mnt/fixture"),
            key_path: None,
            tpm_handle: Some(PersistentHandle::fixture()),
            key_prompt: Some(true),
            no_header: Some(true),
        }
    }

    /// Create an [`Options`] fixture with every key source disabled.
    #[cfg(test)]
    pub fn fixture_without_key_sources() -> Self {
        Self {
            key_path: None,
            tpm_handle: None,
            key_prompt: None,
            ..Self::fixture()
        }
    }
}

impl FromServices for Options {
    type Error = OptionsError;

    fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>>
    where
        Self: Sized,
    {
        let cli = services
            .get::<CliArgs>()
            .change_context(OptionsError::GetCliArgs)?;
        let path = match &cli.config {
            Some(path) => path,
            None => &get_config_path()?,
        };
        Self::from_file(path)
    }
}

fn get_config_path() -> Result<PathBuf, Report<OptionsError>> {
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
    let dir = dirs::config_dir()
        .ok_or_else(|| Report::new(OptionsError::ConfigDir))?
        .join(APP_NAME);
    let paths = fs::read_dir(&dir)
        .change_context(OptionsError::ReadDir)
        .attach_path(&dir)?
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

/// Errors returned by [`Options`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum OptionsError {
    /// Unable to resolve [`CliArgs`].
    #[error("Unable to get CLI arguments")]
    GetCliArgs,
    /// Unable to locate the user config dir.
    #[error("Unable to locate config directory")]
    ConfigDir,
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

    #[test]
    fn options_from_file() {
        // Arrange
        let dir = TempDirectory::default()
            .create()
            .expect("should create temp directory");
        let mut paths = Vec::new();
        for i in 1..=3 {
            let path = dir.join(format!("config{i}.yaml"));
            let content =
                format!("partuuid: uuid-{i}\nmapper_name: test-{i}\nmount_path: /mnt/test{i}\n");
            fs::write(&path, content).expect("should write config file");
            paths.push(path);
        }
        let target_path = paths.get(1).expect("should have path at index 1").clone();

        // Act
        let options = Options::from_file(&target_path).expect("should read options");

        // Assert
        assert_eq!(options.mapper_name, "test-2");
        assert_eq!(options.partuuid, "uuid-2");
        assert_eq!(options.mount_path, PathBuf::from("/mnt/test2"));
    }
}
