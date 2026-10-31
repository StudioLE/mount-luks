//! TPM persistent handle addresses.

use crate::prelude::*;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// TPM persistent handle address.
///
/// - Stores the offset from [`PersistentHandle::BASE`] in 2 bytes instead of 4
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PersistentHandle(u16);

impl PersistentHandle {
    /// Base address for TPM persistent handles (`0x81000000`).
    pub const BASE: u32 = 0x81000000;
    /// Maximum offset from the base address.
    pub const MAX_OFFSET: u16 = u16::MAX;

    /// Create a [`PersistentHandle`] from an offset.
    ///
    /// - Accepts the full `u16` range, `0` to `65535`
    #[must_use]
    pub const fn from_offset(offset: u16) -> Self {
        Self(offset)
    }

    /// Create a [`PersistentHandle`] from a full `u32` address.
    pub fn new(handle: u32) -> Result<Self, PersistentHandleError> {
        if handle < Self::BASE {
            return Err(PersistentHandleError::Min);
        }
        let offset = handle - Self::BASE;
        if offset > u32::from(u16::MAX) {
            return Err(PersistentHandleError::Max);
        }
        Ok(Self(
            u16::try_from(offset).expect("Offset should fit in u16"),
        ))
    }

    /// Full `u32` address of the handle.
    #[expect(clippy::as_conversions, reason = "u16 to u32 widening is always safe")]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        Self::BASE + self.0 as u32
    }

    /// Offset from [`PersistentHandle::BASE`].
    #[must_use]
    pub const fn offset(self) -> u16 {
        self.0
    }

    /// Create a [`PersistentHandle`] fixture for testing.
    #[cfg(test)]
    pub fn fixture() -> Self {
        Self::from_offset(1)
    }
}

impl TryFrom<u32> for PersistentHandle {
    type Error = PersistentHandleError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PersistentHandle> for u32 {
    fn from(handle: PersistentHandle) -> Self {
        handle.as_u32()
    }
}

impl Display for PersistentHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "0x{:08X}", self.as_u32())
    }
}

impl FromStr for PersistentHandle {
    type Err = PersistentHandleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        let hex_str = trimmed
            .strip_prefix("0x")
            .or_else(|| trimmed.strip_prefix("0X"))
            .unwrap_or(trimmed);
        let value = u32::from_str_radix(hex_str, 16).map_err(|_| PersistentHandleError::Parse)?;
        Self::new(value)
    }
}

impl TryFrom<String> for PersistentHandle {
    type Error = PersistentHandleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl TryFrom<&str> for PersistentHandle {
    type Error = PersistentHandleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl Serialize for PersistentHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{:08X}", self.as_u32()))
    }
}

impl<'de> Deserialize<'de> for PersistentHandle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(DeError::custom)
    }
}

/// Errors returned by [`PersistentHandle`] constructors and parsers.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PersistentHandleError {
    /// The address is below [`PersistentHandle::BASE`].
    #[error("Value must be at least 0x81000000")]
    Min,
    /// The offset from [`PersistentHandle::BASE`] doesn't fit in a `u16`.
    #[error("Value must be at most 0x8100FFFF")]
    Max,
    /// The value isn't a hex number.
    #[error("Invalid hex string")]
    Parse,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`PersistentHandle::new`] at the base address.
    #[test]
    fn persistent_handle_new() {
        let handle = PersistentHandle::new(0x81000000).expect("base handle should be valid");
        assert_eq!(handle.as_u32(), 0x81000000);
        assert_eq!(handle.offset(), 0);
    }

    /// [`PersistentHandle::new`] across the valid range, including the max offset.
    #[test]
    fn persistent_handle_new_valid_range() {
        assert!(PersistentHandle::new(0x81000001).is_ok());
        assert!(PersistentHandle::new(0x81000007).is_ok());
        assert!(PersistentHandle::new(0x8100FFFF).is_ok());
        let max_handle = PersistentHandle::new(0x81000000 + u32::from(u16::MAX))
            .expect("max handle should be valid");
        assert_eq!(max_handle.offset(), u16::MAX);
    }

    /// [`PersistentHandle::new`] below the base address.
    #[test]
    fn persistent_handle_new_below_base() {
        assert_eq!(
            PersistentHandle::new(0x80FFFFFF),
            Err(PersistentHandleError::Min)
        );
        assert_eq!(
            PersistentHandle::new(0x00000000),
            Err(PersistentHandleError::Min)
        );
        assert_eq!(
            PersistentHandle::new(0x80000000),
            Err(PersistentHandleError::Min)
        );
    }

    /// [`PersistentHandle::new`] at the bounds named in each [`PersistentHandleError`] message.
    #[test]
    fn persistent_handle_new_error_bounds() {
        assert!(PersistentHandle::new(0x81000000).is_ok());
        assert_eq!(
            PersistentHandle::new(0x80FFFFFF),
            Err(PersistentHandleError::Min)
        );
        assert!(PersistentHandle::new(0x8100FFFF).is_ok());
        assert_eq!(
            PersistentHandle::new(0x81010000),
            Err(PersistentHandleError::Max)
        );
    }

    /// [`PersistentHandle::new`] beyond the max offset.
    #[test]
    fn persistent_handle_new_above_max() {
        assert_eq!(
            PersistentHandle::new(0x81010000),
            Err(PersistentHandleError::Max)
        );
        assert_eq!(
            PersistentHandle::new(0x81FFFFFF),
            Err(PersistentHandleError::Max)
        );
        assert_eq!(
            PersistentHandle::new(0x82000000),
            Err(PersistentHandleError::Max)
        );
    }

    /// [`PersistentHandle::new`] with the value of each [`PersistentHandle::as_u32`].
    #[test]
    fn persistent_handle_new_roundtrip() {
        for offset in [0_u16, 1, 7, 255, 1000, u16::MAX] {
            let handle = PersistentHandle::from_offset(offset);
            let output = PersistentHandle::new(handle.as_u32());
            assert_eq!(output, Ok(handle));
        }
    }

    /// [`PersistentHandle::from_offset`] at the first, an arbitrary, and the last offset.
    #[test]
    fn persistent_handle_from_offset() {
        assert_eq!(PersistentHandle::from_offset(0).as_u32(), 0x81000000);
        assert_eq!(PersistentHandle::from_offset(5).as_u32(), 0x81000005);
        assert_eq!(
            PersistentHandle::from_offset(u16::MAX).as_u32(),
            0x81000000 + u32::from(u16::MAX)
        );
    }

    /// [`PersistentHandle::try_from`] with a valid [`u32`].
    #[test]
    fn persistent_handle_try_from_u32() {
        let output = PersistentHandle::try_from(0x81000003_u32);
        assert_eq!(output, Ok(PersistentHandle::from_offset(3)));
    }

    /// [`PersistentHandle::try_from`] with an out of range [`u32`].
    #[test]
    fn persistent_handle_try_from_u32_invalid() {
        assert!(PersistentHandle::try_from(0x80FFFFFF_u32).is_err());
        assert!(PersistentHandle::try_from(0x81FFFFFF_u32).is_err());
    }

    /// [`u32::from`] a [`PersistentHandle`].
    #[test]
    fn u32_from() {
        let output = u32::from(PersistentHandle::from_offset(42));
        assert_eq!(output, 0x81000000 + 42);
    }

    /// [`PersistentHandle`] formatted as the hex string passed to `tpm2-tools`.
    #[test]
    fn persistent_handle_display() {
        assert_eq!(PersistentHandle::from_offset(0).to_string(), "0x81000000");
        assert_eq!(PersistentHandle::from_offset(1).to_string(), "0x81000001");
        assert_eq!(
            PersistentHandle::from_offset(0x10).to_string(),
            "0x81000010"
        );
        assert_eq!(
            PersistentHandle::from_offset(0xABCD).to_string(),
            "0x8100ABCD"
        );
    }

    /// [`PersistentHandle::from_str`] with each [`Display`] output.
    #[test]
    fn persistent_handle_display_roundtrip() {
        for offset in [0_u16, 1, 7, 255, 1000, u16::MAX] {
            let handle = PersistentHandle::from_offset(offset);
            let output = PersistentHandle::from_str(&handle.to_string());
            assert_eq!(output, Ok(handle));
        }
    }

    /// [`PersistentHandle::from_str`] with a lowercase and an uppercase `0x` prefix.
    #[test]
    fn persistent_handle_from_str_with_prefix() {
        assert_eq!(
            PersistentHandle::from_str("0x81000003"),
            Ok(PersistentHandle::from_offset(3))
        );
        assert_eq!(
            PersistentHandle::from_str("0X81000001"),
            Ok(PersistentHandle::from_offset(1))
        );
    }

    /// [`PersistentHandle::from_str`] without a `0x` prefix.
    #[test]
    fn persistent_handle_from_str_without_prefix() {
        let output = PersistentHandle::from_str("81000003");
        assert_eq!(output, Ok(PersistentHandle::from_offset(3)));
    }

    /// [`PersistentHandle::from_str`] with surrounding whitespace.
    #[test]
    fn persistent_handle_from_str_with_whitespace() {
        let output = PersistentHandle::from_str("  0x81000003  ");
        assert_eq!(output, Ok(PersistentHandle::from_offset(3)));
    }

    /// [`PersistentHandle::from_str`] with values that aren't hex.
    #[test]
    fn persistent_handle_from_str_invalid() {
        assert_eq!(
            PersistentHandle::from_str("invalid"),
            Err(PersistentHandleError::Parse)
        );
        assert_eq!(
            PersistentHandle::from_str("0xGGGGGG"),
            Err(PersistentHandleError::Parse)
        );
        assert_eq!(
            PersistentHandle::from_str(""),
            Err(PersistentHandleError::Parse)
        );
    }

    /// [`PersistentHandle::from_str`] with hex values outside the range.
    #[test]
    fn persistent_handle_from_str_out_of_range() {
        assert_eq!(
            PersistentHandle::from_str("0x80FFFFFF"),
            Err(PersistentHandleError::Min)
        );
        assert_eq!(
            PersistentHandle::from_str("0x81FFFFFF"),
            Err(PersistentHandleError::Max)
        );
    }

    /// [`PersistentHandle::try_from`] with a [`String`].
    #[test]
    fn persistent_handle_try_from_string() {
        let output = PersistentHandle::try_from(String::from("0x81000003"));
        assert_eq!(output, Ok(PersistentHandle::from_offset(3)));
    }

    /// [`PersistentHandle::try_from`] with a `&str` with and without a prefix.
    #[test]
    fn persistent_handle_try_from_str() {
        assert_eq!(
            PersistentHandle::try_from("0x81000003"),
            Ok(PersistentHandle::from_offset(3))
        );
        assert_eq!(
            PersistentHandle::try_from("81000003"),
            Ok(PersistentHandle::from_offset(3))
        );
    }

    /// [`PersistentHandle::serialize`] to a quoted YAML hex string.
    #[test]
    fn persistent_handle_serialize() {
        for (offset, expected) in [
            (3, "'0x81000003'\n"),
            (0, "'0x81000000'\n"),
            (0xABCD, "'0x8100ABCD'\n"),
        ] {
            let handle = PersistentHandle::from_offset(offset);
            let output = serde_yaml::to_string(&handle).expect("handle should serialize to YAML");
            assert_eq!(output, expected);
        }
    }

    /// [`PersistentHandle::deserialize`] with a lowercase and an uppercase `0x` prefix.
    #[test]
    fn persistent_handle_deserialize_with_prefix() {
        let output: PersistentHandle =
            serde_yaml::from_str(r#""0x81000003""#).expect("should deserialize");
        assert_eq!(output, PersistentHandle::from_offset(3));
        let output: PersistentHandle =
            serde_yaml::from_str(r#""0X81000001""#).expect("should deserialize");
        assert_eq!(output, PersistentHandle::from_offset(1));
    }

    /// [`PersistentHandle::deserialize`] without a `0x` prefix.
    #[test]
    fn persistent_handle_deserialize_without_prefix() {
        let output: PersistentHandle =
            serde_yaml::from_str(r#""81000003""#).expect("should deserialize");
        assert_eq!(output, PersistentHandle::from_offset(3));
    }

    /// [`PersistentHandle::deserialize`] with surrounding whitespace.
    #[test]
    fn persistent_handle_deserialize_with_whitespace() {
        let output: PersistentHandle =
            serde_yaml::from_str(r#""  0x81000003  ""#).expect("should deserialize");
        assert_eq!(output, PersistentHandle::from_offset(3));
    }

    /// [`PersistentHandle::deserialize`] with invalid, out of range, and empty values.
    #[test]
    fn persistent_handle_deserialize_invalid() {
        for yaml in [
            r#""invalid""#,
            r#""0x80FFFFFF""#,
            r#""0x81FFFFFF""#,
            r#""""#,
        ] {
            let output = serde_yaml::from_str::<PersistentHandle>(yaml);
            assert!(output.is_err(), "{yaml} should not deserialize");
        }
    }

    /// [`PersistentHandle::deserialize`] with each [`PersistentHandle::serialize`] output.
    #[test]
    fn persistent_handle_serde_roundtrip() {
        for offset in [0_u16, 1, 7, 255, 1000, u16::MAX] {
            let handle = PersistentHandle::from_offset(offset);
            let yaml = serde_yaml::to_string(&handle).expect("handle should serialize");
            let output: PersistentHandle =
                serde_yaml::from_str(&yaml).expect("serialized handle should deserialize");
            assert_eq!(output, handle);
        }
    }
}
