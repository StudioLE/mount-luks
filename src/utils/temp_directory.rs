#![expect(
    dead_code,
    reason = "test-only module, not all helpers used in every test"
)]
use crate::prelude::APP_NAME;
use chrono::Local;
use std::env::temp_dir;
use std::fs::create_dir_all;
use std::io::Error as IoError;
use std::path::PathBuf;
use std::thread::current as current_thread;

/// Builder for a temporary directory path used in tests.
pub struct TempDirectory {
    path: PathBuf,
}

impl TempDirectory {
    /// Create a new [`TempDirectory`] rooted at the system temp directory.
    #[must_use]
    pub fn new() -> TempDirectory {
        Self { path: temp_dir() }
    }

    /// Append the application name as a path component.
    #[must_use]
    pub fn with_name(mut self) -> TempDirectory {
        self.path.push(APP_NAME);
        self
    }

    /// Append the current datetime as a path component.
    #[must_use]
    pub fn with_datetime(mut self) -> TempDirectory {
        self.path.push(get_datetime());
        self
    }

    /// Append the current test name as a path component.
    #[must_use]
    pub fn with_test_name(mut self) -> TempDirectory {
        self.path.push(get_test_name_underscored());
        self
    }

    /// Return the built path without creating the directory.
    #[must_use]
    pub fn get_path(self) -> PathBuf {
        self.path
    }

    /// Create the directory and return its path.
    pub fn create(self) -> Result<PathBuf, IoError> {
        create_dir_all(&self.path)?;
        Ok(self.path)
    }
}

impl Default for TempDirectory {
    fn default() -> Self {
        Self::new().with_name().with_test_name().with_datetime()
    }
}

fn get_datetime() -> String {
    Local::now().to_rfc3339().replace(':', "_").clone()
}

fn get_test_name_underscored() -> String {
    current_thread()
        .name()
        .expect("should be able to get test name")
        .replace(':', "_")
}
