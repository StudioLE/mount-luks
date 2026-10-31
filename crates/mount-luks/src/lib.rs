//! CLI tool for unlocking and mounting LUKS encrypted partitions.

mod app;
mod commands;
mod extensions;
pub mod prelude;
mod services;
#[cfg(test)]
mod testing;
