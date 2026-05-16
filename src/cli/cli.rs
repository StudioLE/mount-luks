use crate::prelude::*;
use std::process::ExitCode;

/// Application entrypoint that bootstraps services and runs a subcommand.
pub struct Cli {
    services: ServiceProvider,
}

impl Cli {
    /// Create a new [`Cli`] with the default service registrations.
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: ServiceBuilder::new()
                .with_app_services()
                .build()
                .expect_init(),
        }
    }

    /// Run the CLI to completion, returning the appropriate exit code.
    #[must_use]
    pub fn run(&self) -> ExitCode {
        if let Err(e) = self.run_subcommand() {
            Ui::error("Unable to continue", &e);
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }

    fn run_subcommand(&self) -> Result<(), StructuredError> {
        let handler = self.services.expect::<SubCommandHandler>();
        handler.run()
    }
}
