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
            services: ServiceBuilder::new().with_app_services().build(),
        }
    }

    /// Run the CLI to completion, returning the appropriate exit code.
    #[must_use]
    pub fn run(&self) -> ExitCode {
        self.init_logger();
        if let Err(e) = self.run_subcommand() {
            Logger::error("Unable to continue", &e);
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }

    fn init_logger(&self) {
        let logger = self
            .services
            .get::<Logger>()
            .expect("should be able to resolve Logger");
        logger.init();
    }

    fn run_subcommand(&self) -> Result<(), StructuredError> {
        let handler = self
            .services
            .get::<SubCommandHandler>()
            .expect("should be able to resolve SubCommandHandler");
        handler.run()
    }
}
