use crate::prelude::*;

const MAX_PERSISTENT_HANDLES: u32 = 7;

#[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
pub fn next_handle(
    mut handles: Vec<PersistentHandle>,
) -> Result<PersistentHandle, Report<NextHandleError>> {
    handles.sort_unstable();
    for i in 0..MAX_PERSISTENT_HANDLES {
        let candidate =
            PersistentHandle::from_offset(u16::try_from(i).expect("Offset should fit in u16"));
        if !handles.contains(&candidate) {
            return Ok(candidate);
        }
    }
    Err(Report::new(NextHandleError))
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Unable to find next available TPM persistent handle")]
pub struct NextHandleError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _get_next_handle() {
        // Arrange
        let handles = vec![
            PersistentHandle::new(0x81000004).expect("handle should be valid"),
            PersistentHandle::new(0x81000001).expect("handle should be valid"),
            PersistentHandle::new(0x81000005).expect("handle should be valid"),
        ];

        // Act
        let next = next_handle(handles).expect("should be able to get next handle");

        // Assert
        assert_eq!(
            next,
            PersistentHandle::new(0x81000000).expect("handle should be valid")
        );
    }
}
