use crate::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Compact representation of a TPM persistent handle (2 bytes instead of 4)
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PersistentHandle(u16);

impl PersistentHandle {
    pub const BASE: u32 = 0x81000000;
    pub const MAX_OFFSET: u16 = u16::MAX;

    /// Creates a handle from an offset (0-65535)
    #[must_use]
    pub const fn from_offset(offset: u16) -> Self {
        Self(offset)
    }

    /// Creates a handle from a full u32 value
    #[allow(clippy::as_conversions)]
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

    /// Get the full u32 handle value
    #[allow(clippy::as_conversions)]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        Self::BASE + self.0 as u32
    }

    /// Get the offset from base
    #[must_use]
    pub const fn offset(self) -> u16 {
        self.0
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
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        value.parse()
    }
}

impl TryFrom<&str> for PersistentHandle {
    type Error = PersistentHandleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
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
    #[allow(clippy::absolute_paths)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum PersistentHandleError {
    #[error("Value must be greater than 0x81000000")]
    Min,
    #[error("Value must be less than 0x81FFFFFF")]
    Max,
    #[error("Invalid hex string")]
    Parse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_at_base() {
        let handle = PersistentHandle::new(0x81000000).expect("Base handle should be valid");
        assert_eq!(handle.as_u32(), 0x81000000);
        assert_eq!(handle.offset(), 0);
    }

    #[test]
    fn test_new_valid_range() {
        // Test various valid handles
        assert!(PersistentHandle::new(0x81000000).is_ok());
        assert!(PersistentHandle::new(0x81000001).is_ok());
        assert!(PersistentHandle::new(0x81000007).is_ok());
        assert!(PersistentHandle::new(0x8100FFFF).is_ok());

        // Test at max offset (0x81000000 + 0xFFFF)
        let max_handle = PersistentHandle::new(0x81000000 + u32::from(u16::MAX))
            .expect("Max valid handle should be accepted");
        assert_eq!(max_handle.offset(), u16::MAX);
    }

    #[test]
    fn test_new_below_base() {
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

    #[test]
    fn test_new_above_max() {
        // 0x81010000 is beyond u16::MAX offset
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

    #[test]
    fn test_from_offset() {
        let handle = PersistentHandle::from_offset(0);
        assert_eq!(handle.as_u32(), 0x81000000);

        let handle = PersistentHandle::from_offset(5);
        assert_eq!(handle.as_u32(), 0x81000005);

        let handle = PersistentHandle::from_offset(u16::MAX);
        assert_eq!(handle.as_u32(), 0x81000000 + u32::from(u16::MAX));
    }

    #[test]
    fn test_offset() {
        assert_eq!(PersistentHandle::from_offset(0).offset(), 0);
        assert_eq!(PersistentHandle::from_offset(1).offset(), 1);
        assert_eq!(PersistentHandle::from_offset(255).offset(), 255);
        assert_eq!(PersistentHandle::from_offset(u16::MAX).offset(), u16::MAX);
    }

    #[test]
    fn test_try_from_u32_valid() {
        let handle =
            PersistentHandle::try_from(0x81000003).expect("Valid handle should convert from u32");
        assert_eq!(handle.as_u32(), 0x81000003);
        assert_eq!(handle.offset(), 3);
    }

    #[test]
    fn test_try_from_u32_invalid() {
        assert!(PersistentHandle::try_from(0x80FFFFFF).is_err());
        assert!(PersistentHandle::try_from(0x81FFFFFF).is_err());
    }

    #[test]
    fn test_into_u32() {
        let handle = PersistentHandle::from_offset(42);
        let value: u32 = handle.into();
        assert_eq!(value, 0x81000000 + 42);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", PersistentHandle::from_offset(0)),
            "0x81000000"
        );
        assert_eq!(
            format!("{}", PersistentHandle::from_offset(1)),
            "0x81000001"
        );
        assert_eq!(
            format!("{}", PersistentHandle::from_offset(0x10)),
            "0x81000010"
        );
        assert_eq!(
            format!("{}", PersistentHandle::from_offset(0xABCD)),
            "0x8100ABCD"
        );
    }

    #[test]
    fn test_comparison() {
        let h0 = PersistentHandle::from_offset(0);
        let h1 = PersistentHandle::from_offset(1);
        let h2 = PersistentHandle::from_offset(2);
        let h1_dup = PersistentHandle::from_offset(1);

        assert!(h0 < h1);
        assert!(h1 < h2);
        assert!(h2 > h1);
        assert!(h1 <= h1_dup);
        assert!(h1 >= h1_dup);
        assert_eq!(h1, h1_dup);
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_ordering() {
        let mut handles = [
            PersistentHandle::from_offset(5),
            PersistentHandle::from_offset(1),
            PersistentHandle::from_offset(3),
            PersistentHandle::from_offset(0),
        ];

        handles.sort();

        assert_eq!(handles[0].offset(), 0);
        assert_eq!(handles[1].offset(), 1);
        assert_eq!(handles[2].offset(), 3);
        assert_eq!(handles[3].offset(), 5);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        let h1 = PersistentHandle::from_offset(1);
        let h1_dup = PersistentHandle::from_offset(1);
        let h2 = PersistentHandle::from_offset(2);

        set.insert(h1);
        assert!(set.contains(&h1_dup)); // Same value should be found
        assert!(!set.contains(&h2));

        set.insert(h2);
        assert_eq!(set.len(), 2); // Only 2 unique values
    }

    #[test]
    fn test_roundtrip() {
        for offset in [0_u16, 1, 7, 255, 1000, u16::MAX] {
            let handle = PersistentHandle::from_offset(offset);
            let value = handle.as_u32();
            let reconstructed =
                PersistentHandle::new(value).expect("Converted u32 should create valid handle");
            assert_eq!(handle, reconstructed);
            assert_eq!(reconstructed.offset(), offset);
        }
    }

    #[test]
    fn test_size() {
        // Verify the struct is actually 2 bytes
        assert_eq!(size_of::<PersistentHandle>(), 2);
        assert_eq!(size_of::<Option<PersistentHandle>>(), 4);
    }

    #[test]
    fn test_debug_output() {
        let handle = PersistentHandle::from_offset(42);
        let debug_str = format!("{handle:?}");
        assert!(debug_str.contains("PersistentHandle"));
    }

    #[test]
    fn test_boundary_conditions() {
        // Exactly at boundaries
        assert!(PersistentHandle::new(0x81000000).is_ok()); // Min valid
        assert!(PersistentHandle::new(0x80FFFFFF).is_err()); // Just below min

        let max_valid = 0x81000000 + u32::from(u16::MAX);
        assert!(PersistentHandle::new(max_valid).is_ok()); // Max valid
        assert!(PersistentHandle::new(max_valid + 1).is_err()); // Just above max
    }

    #[test]
    fn test_from_str_with_prefix() {
        let handle: PersistentHandle = "0x81000003"
            .parse()
            .expect("Valid hex string with prefix should parse");
        assert_eq!(handle.as_u32(), 0x81000003);

        let handle: PersistentHandle = "0X81000001"
            .parse()
            .expect("Valid hex string with uppercase prefix should parse");
        assert_eq!(handle.as_u32(), 0x81000001);
    }

    #[test]
    fn test_from_str_without_prefix() {
        let handle: PersistentHandle = "81000003"
            .parse()
            .expect("Valid hex string without prefix should parse");
        assert_eq!(handle.as_u32(), 0x81000003);
    }

    #[test]
    fn test_from_str_with_whitespace() {
        let handle: PersistentHandle = "  0x81000003  "
            .parse()
            .expect("Valid hex string with whitespace should parse");
        assert_eq!(handle.as_u32(), 0x81000003);
    }

    #[test]
    fn test_from_str_invalid() {
        assert_eq!(
            "invalid".parse::<PersistentHandle>(),
            Err(PersistentHandleError::Parse)
        );
        assert_eq!(
            "0xGGGGGG".parse::<PersistentHandle>(),
            Err(PersistentHandleError::Parse)
        );
        assert_eq!(
            "".parse::<PersistentHandle>(),
            Err(PersistentHandleError::Parse)
        );
    }

    #[test]
    fn test_from_str_out_of_range() {
        assert_eq!(
            "0x80FFFFFF".parse::<PersistentHandle>(),
            Err(PersistentHandleError::Min)
        );
        assert_eq!(
            "0x81FFFFFF".parse::<PersistentHandle>(),
            Err(PersistentHandleError::Max)
        );
    }

    #[test]
    fn test_display_from_str_roundtrip() {
        for offset in [0_u16, 1, 7, 255, 1000, u16::MAX] {
            let handle = PersistentHandle::from_offset(offset);
            let string = handle.to_string();
            let parsed: PersistentHandle = string
                .parse()
                .expect("Display output should parse back to same handle");
            assert_eq!(handle, parsed);
        }
    }

    #[test]
    fn test_try_from_string() {
        let s = "0x81000003".to_owned();
        let handle = PersistentHandle::try_from(s).expect("Valid string should convert to handle");
        assert_eq!(handle.as_u32(), 0x81000003);
    }

    #[test]
    fn test_try_from_str() {
        let handle =
            PersistentHandle::try_from("0x81000003").expect("Valid str with prefix should convert");
        assert_eq!(handle.as_u32(), 0x81000003);

        let handle = PersistentHandle::try_from("81000003")
            .expect("Valid str without prefix should convert");
        assert_eq!(handle.as_u32(), 0x81000003);
    }

    #[test]
    fn test_serialize_to_string() {
        let handle = PersistentHandle::from_offset(3);
        let json = serde_yaml::to_string(&handle).expect("Handle should serialize to YAML");
        assert_eq!(json, "'0x81000003'\n");

        let handle = PersistentHandle::from_offset(0);
        let json = serde_yaml::to_string(&handle).expect("Handle should serialize to YAML");
        assert_eq!(json, "'0x81000000'\n");

        let handle = PersistentHandle::from_offset(0xABCD);
        let json = serde_yaml::to_string(&handle).expect("Handle should serialize to YAML");
        assert_eq!(json, "'0x8100ABCD'\n");
    }

    #[test]
    fn test_deserialize_from_string_with_prefix() {
        let json = r#""0x81000003""#;
        let handle: PersistentHandle =
            serde_yaml::from_str(json).expect("Valid YAML hex string should deserialize");
        assert_eq!(handle.as_u32(), 0x81000003);

        let json = r#""0X81000001""#; // Uppercase X
        let handle: PersistentHandle =
            serde_yaml::from_str(json).expect("Valid YAML hex string should deserialize");
        assert_eq!(handle.as_u32(), 0x81000001);
    }

    #[test]
    fn test_deserialize_from_string_without_prefix() {
        let json = r#""81000003""#;
        let handle: PersistentHandle =
            serde_yaml::from_str(json).expect("Valid YAML hex string should deserialize");
        assert_eq!(handle.as_u32(), 0x81000003);
    }

    #[test]
    fn test_deserialize_invalid_string() {
        // Invalid hex
        let json = r#""invalid""#;
        assert!(serde_yaml::from_str::<PersistentHandle>(json).is_err());

        // Out of range (below)
        let json = r#""0x80FFFFFF""#;
        assert!(serde_yaml::from_str::<PersistentHandle>(json).is_err());

        // Out of range (above)
        let json = r#""0x81FFFFFF""#;
        assert!(serde_yaml::from_str::<PersistentHandle>(json).is_err());

        // Empty string
        let json = r#""""#;
        assert!(serde_yaml::from_str::<PersistentHandle>(json).is_err());
    }

    #[test]
    fn test_serde_roundtrip() {
        for offset in [0_u16, 1, 7, 255, 1000, u16::MAX] {
            let original = PersistentHandle::from_offset(offset);
            let json = serde_yaml::to_string(&original).expect("Handle should serialize");
            let deserialized: PersistentHandle =
                serde_yaml::from_str(&json).expect("Serialized handle should deserialize");
            assert_eq!(original, deserialized);
        }
    }

    #[test]
    fn test_deserialize_with_whitespace() {
        let json = r#""  0x81000003  ""#;
        let handle: PersistentHandle =
            serde_yaml::from_str(json).expect("Valid YAML hex string should deserialize");
        assert_eq!(handle.as_u32(), 0x81000003);
    }

    #[test]
    fn test_serialize_vec() {
        let handles = vec![
            PersistentHandle::from_offset(0),
            PersistentHandle::from_offset(1),
            PersistentHandle::from_offset(2),
        ];

        let json = serde_yaml::to_string(&handles).expect("Vec of handles should serialize");
        assert!(json.contains("'0x81000000'"));
        assert!(json.contains("'0x81000001'"));
        assert!(json.contains("'0x81000002'"));

        let deserialized: Vec<PersistentHandle> =
            serde_yaml::from_str(&json).expect("Serialized handle should deserialize");
        assert_eq!(deserialized, handles);
    }
}
