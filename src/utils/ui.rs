use crate::prelude::*;
use owo_colors::OwoColorize;

const CHECK: &str = " ✓ ";
const CROSS: &str = " ⨯ ";

/// Print the application header with the current options and command.
pub fn print_header(options: &Options, command: SubCommand) {
    let title = [
        "╭────────────────────────────────────────────────╮",
        "│ Unlock and mount a LUKS partition              │",
        "╰────────────────────────────────────────────────╯",
    ];
    let body = [
        format!("     Command: {command}"),
        format!("   Partition: {}", options.partuuid),
        format!(" Mapper path: {}", options.get_mapper_path().display()),
        format!("  Mount path: {}", options.mount_path.display()),
        format!("    Key path: {}", display_path_option(&options.key_path)),
        format!("  TPM handle: {}", display_option(&options.tpm_handle)),
        format!("  Key prompt: {}", display_option(&options.key_prompt)),
    ];
    eprintln!(
        "{}\n{}\n",
        title.join("\n").bold(),
        body.join("\n").dimmed()
    );
}

/// Print the start of a step, incrementing the shared counter.
pub fn print_step_start(mut_counter: &Mutex<usize>, total_steps: usize, message: &str) {
    let mut i = mut_counter.lock().expect("Should be able to lock mutex");
    *i += 1;
    info!("{}", format!("{i}/{total_steps} {message}").dimmed());
}

/// Print a success indicator for a completed step.
pub fn print_step_completed(message: &str) {
    info!("{} {message}", CHECK.dimmed());
}

/// Print an error indicator with the given message.
pub fn print_error(message: &str) {
    error!("{} {message}", CROSS.dimmed());
}

#[expect(
    clippy::ref_option,
    reason = "display helper intentionally borrows the Option"
)]
fn display_option<T: Display>(value: &Option<T>) -> String {
    if let Some(value) = value {
        value.to_string()
    } else {
        "None".italic().to_string()
    }
}

#[expect(
    clippy::ref_option,
    reason = "display helper intentionally borrows the Option"
)]
fn display_path_option(value: &Option<PathBuf>) -> String {
    if let Some(value) = value {
        value.display().to_string()
    } else {
        "None".italic().to_string()
    }
}
