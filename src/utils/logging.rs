use std::io::stderr;
use std::sync::OnceLock;
use std::time::Instant;

use clap::ValueEnum;
use strum::Display;
use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing::subscriber::set_global_default;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};

static INIT: OnceLock<()> = OnceLock::new();

pub fn init_elapsed_logger(log_level: Option<LogLevel>) {
    let filter = LevelFilter::from(log_level.unwrap_or_default());
    INIT.get_or_init(|| {
        let targets = get_targets().with_default(filter);
        let layer = layer()
            .compact()
            .with_writer(stderr)
            .with_target(false)
            .with_timer(ElapsedTime::default())
            .with_filter(targets);
        let registry = Registry::default().with(layer);
        set_global_default(registry).expect("should be able to set global default");
    });
}

#[must_use]
pub fn get_targets() -> Targets {
    Targets::new()
}

#[derive(Copy, Clone, Default, Debug, ValueEnum, Display)]
#[strum(serialize_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => Level::ERROR,
            LogLevel::Warn => Level::WARN,
            LogLevel::Info => Level::INFO,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Trace => Level::TRACE,
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        LevelFilter::from_level(Level::from(level))
    }
}

struct ElapsedTime {
    start: Instant,
}

impl Default for ElapsedTime {
    fn default() -> Self {
        ElapsedTime {
            start: Instant::now(),
        }
    }
}

impl FormatTime for ElapsedTime {
    #[allow(clippy::absolute_paths)]
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let elapsed = self.start.elapsed();
        write!(w, "{:.3}", elapsed.as_secs_f64())
    }
}
