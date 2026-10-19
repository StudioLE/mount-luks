use crate::prelude::*;
use std::convert::Infallible;
use usage_rs::Cli as UsageCli;

/// Parsed CLI arguments.
#[derive(Debug, UsageCli)]
#[usage(bin = "mount-luks", about, version, unknown_flags = "error")]
pub struct CliArgs {
    /// Path to a specific config file
    #[usage(long)]
    pub config: Option<PathBuf>,

    /// Set the log level
    #[usage(long, value_enum)]
    pub log_level: Option<LogLevel>,

    /// Subcommand to execute
    #[usage(subcommand)]
    pub command: Option<Subcommand>,
}

impl FromServices for CliArgs {
    type Error = Infallible;

    fn from_services(_: &ServiceProvider) -> Result<Self, Report<Self::Error>>
    where
        Self: Sized,
    {
        Ok(CliArgs::parse())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use usage_rs::test::{Page, argv, help, parse};

    /// [`CliArgs::parse_from`] binds a config path, a log level, and a subcommand.
    #[test]
    fn cli_args_parse_from() {
        // Arrange
        let input = argv([
            "--config",
            "/tmp/config.yaml",
            "--log-level",
            "debug",
            "set-tpm",
        ]);
        let words = input.words();
        let spec = CliArgs::spec();
        // Act
        let cli = parse(spec, &words, CliArgs::parse_from).expect("the command line should parse");
        // Assert
        assert_eq!(cli.config, Some(PathBuf::from("/tmp/config.yaml")));
        assert_eq!(cli.log_level, Some(LogLevel::Debug));
        assert_eq!(cli.command, Some(Subcommand::SetTpm));
    }

    /// [`CliArgs::parse_from`] leaves the subcommand unset when none is given.
    #[test]
    fn cli_args_parse_from_without_subcommand() {
        // Arrange
        let input = argv(["--log-level", "info"]);
        let words = input.words();
        let spec = CliArgs::spec();
        // Act
        let cli = parse(spec, &words, CliArgs::parse_from).expect("the command line should parse");
        // Assert
        assert_eq!(cli.command, None);
    }

    /// [`CliArgs::parse_from`] rejects a log level outside the declared choices.
    #[test]
    fn cli_args_parse_from_invalid_log_level() {
        // Arrange
        let input = argv(["--log-level", "nonsense"]);
        let words = input.words();
        let spec = CliArgs::spec();
        // Act
        let message =
            parse(spec, &words, CliArgs::parse_from).expect_err("`nonsense` is not a log level");
        // Assert
        assert!(message.contains("invalid value 'nonsense'"), "{message}");
    }

    /// [`CliArgs::parse_from`] rejects an unknown flag.
    #[test]
    fn cli_args_parse_from_unknown_flag() {
        // Arrange
        let input = argv(["mount", "--oops"]);
        let words = input.words();
        let spec = CliArgs::spec();
        // Act
        let message = parse(spec, &words, CliArgs::parse_from)
            .expect_err("`--oops` is not a flag of `mount`");
        // Assert
        assert!(message.contains("--oops"), "{message}");
    }

    /// The root help page.
    ///
    /// - Replaces the version with a placeholder, as CI sets it at build time
    #[test]
    fn cli_args_help() {
        let spec = CliArgs::spec();
        let output = help(spec, &[], Page::Long).replace(env!("CARGO_PKG_VERSION"), "x.y.z");
        assert_snapshot!(output);
    }
}
