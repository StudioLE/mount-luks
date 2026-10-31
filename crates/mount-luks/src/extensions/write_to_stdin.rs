use std::io::Write;
use std::process::Child;

/// Write a string to the stdin of a child process.
pub trait WriteToStdin {
    /// Write `input` to stdin and return `self`.
    fn write_to_stdin(self, input: &str) -> Self;
}

impl WriteToStdin for Child {
    fn write_to_stdin(mut self, input: &str) -> Self {
        write_to_stdin(&mut self, input);
        self
    }
}

fn write_to_stdin(child: &mut Child, input: &str) {
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .expect("Should be able to write to stdin");
    }
}
