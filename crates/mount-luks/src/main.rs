//! Binary entrypoint for `mount-luks`.

use mount_luks::prelude::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    Cli::new().run()
}
