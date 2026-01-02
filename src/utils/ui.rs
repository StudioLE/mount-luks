use crate::prelude::*;
use owo_colors::OwoColorize;

const CHECK: &str = " ✓ ";
const CROSS: &str = " ⨯ ";

pub fn print_header(options: &Options) {
    let title = [
        "╭────────────────────────────────────────────────╮",
        "│ Unlock and mount a LUKS partition              │",
        "╰────────────────────────────────────────────────╯",
    ];
    let body = [
        format!("   Partition: {} ", options.partition_path.display()),
        format!(" Mapper path: {}", options.get_mapper_path().display()),
        format!("  Mount path: {}", options.mount_path.display()),
        format!(
            "    Key path: {}",
            options
                .key_path
                .clone()
                .map_or(String::new(), |path| path.display().to_string())
        ),
        format!(
            "  TPM handle: {}",
            options
                .tpm_handle
                .map_or(String::new(), |handle| handle.to_string())
        ),
        format!(
            "  Key prompt: {}",
            options
                .key_prompt
                .map_or(String::new(), |handle| handle.to_string())
        ),
    ];
    eprintln!(
        "{}\n{}\n",
        title.join("\n").bold(),
        body.join("\n").dimmed()
    );
}

pub fn print_step_start(mut_counter: &Mutex<usize>, total_steps: usize, message: &str) {
    let mut i = mut_counter.lock().expect("Should be able to lock mutex");
    *i += 1;
    info!("{}", format!("{i}/{total_steps} {message}").dimmed());
}

pub fn print_step_completed(message: &str) {
    info!("{} {message}", CHECK.dimmed());
}

pub fn print_error(message: &str) {
    error!("{} {message}", CROSS.dimmed());
}
