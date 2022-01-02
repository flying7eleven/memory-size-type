//! # Memory Size Type
//!
//! This crate implements a data type (`MemorySize`) for strongly typed memory size indications.
//!
//! There is support for memory units with a base of 10 (as recommended by the International
//! Electrotechnical Commission). A support for memory units with the base of 2 (as standardized
//! by IEC 80000-13) will follow soon.
#![doc(html_root_url = "https://docs.rs/memory-size-type/latest")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

/// The structure for representing a specific number of bytes.
pub struct Byte(u64);

/// The structure for representing a specific number of bytes.
pub type Bytes = Byte;

#[cfg(feature = "std")]
impl std::fmt::Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} byte(s)", self.0)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} byte(s)", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::Byte;

    #[test]
    #[cfg(feature = "std")]
    fn printing_bytes_works_correctly() {
        let bytes_lower_limit = Byte(0);
        let bytes_middle = Byte(512);
        let bytes_upper_limit = Byte(1023);

        assert_eq!(bytes_lower_limit.to_string(), "0 byte(s)");
        assert_eq!(bytes_middle.to_string(), "512 byte(s)");
        assert_eq!(bytes_upper_limit.to_string(), "1023 byte(s)");
    }

    #[test]
    #[cfg(feature = "std")]
    fn printing_kibibytes_works_correctly() {
        let kbytes_lower_limit = Byte(1024);
        let kbytes_middle = Byte(524288);
        let kbytes_upper_limit = Byte(1048575);

        assert_eq!(kbytes_lower_limit.to_string(), "1 kibibyte(s)");
        assert_eq!(kbytes_middle.to_string(), "512 kibibyte(s)");
        assert_eq!(kbytes_upper_limit.to_string(), "1023 kibibyte(s)");
    }

    #[test]
    #[cfg(feature = "std")]
    fn printing_mebibytes_works_correctly() {
        let mbytes_lower_limit = Byte(1048576);
        let mbytes_middle = Byte(536870912);
        let mbytes_upper_limit = Byte(1073741823);

        assert_eq!(mbytes_lower_limit.to_string(), "1 mebibyte(s)");
        assert_eq!(mbytes_middle.to_string(), "512 mebibyte(s)");
        assert_eq!(mbytes_upper_limit.to_string(), "1023 mebibyte(s)");
    }

    #[test]
    #[cfg(feature = "std")]
    fn printing_gibibytes_works_correctly() {
        let gbytes_lower_limit = Byte(1073741824);
        let gbytes_middle = Byte(549755813888);
        let gbytes_upper_limit = Byte(1099511627775);

        assert_eq!(gbytes_lower_limit.to_string(), "1 gibibyte(s)");
        assert_eq!(gbytes_middle.to_string(), "512 gibibyte(s)");
        assert_eq!(gbytes_upper_limit.to_string(), "1023 gibibyte(s)");
    }
}
