//! # Memory Size Type
//!
//! This crate implements several data types for strongly typed memory size indications.
//!
//! There is support for memory units with a base of 10 (as recommended by the International
//! Electrotechnical Commission). A support for memory units with the base of 2 (as standardized
//! by IEC 80000-13) will follow soon.
#![doc(html_root_url = "https://docs.rs/memory-size-type/latest")]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

/// The structure for representing a specific number of bytes.
pub struct Byte {
    /// The internally used value to store the number of bytes which are represented by the instance
    /// of the struct.
    bytes: u64,
}

impl Byte {
    /// Number of bytes in one Kibibyte (KiB).
    const BYTES_IN_ONE_KIBIBYTE: u64 = 1000;
    /// Number of bytes in one Mebibyte (MiB).
    const BYTES_IN_ONE_MEBIBYTE: u64 = Byte::BYTES_IN_ONE_KIBIBYTE * 1000;
    /// Number of bytes in one Gibibyte (GiB).
    const BYTES_IN_ONE_GIBIBYTE: u64 = Byte::BYTES_IN_ONE_MEBIBYTE * 1000;
    /// Number of bytes in one Tebibyte (TiB).
    const BYTES_IN_ONE_TEBIBYTE: u64 = Byte::BYTES_IN_ONE_GIBIBYTE * 1000;

    /// Get the string representation for the represented value.
    ///
    /// The value will use the correct SI-unit abbreviation to display the value. See more on
    /// that topic on <https://en.wikipedia.org/wiki/Byte#Multiple-byte_units>.
    ///
    /// # Panics
    /// Will panic if the represented value is larger than 1.099.511.627.775 (Tibibyte).
    ///
    /// # Examples
    /// ```rust
    /// use memory_size_type::Byte;
    /// let some_value = Byte::from(8123);
    ///
    /// assert_eq!(some_value.to_string(), "8.123 KiB");
    /// ```
    fn get_string_representation(&self) -> String {
        // if it's less than a kibibyte, return the bytes
        if self.bytes < Byte::BYTES_IN_ONE_KIBIBYTE {
            return format!("{:} B", self.bytes);
        }

        // if it's less than a mebibyte, return it as kibibyte
        if self.bytes < Bytes::BYTES_IN_ONE_MEBIBYTE {
            let bytes_to_display = self.bytes as f64 / Byte::BYTES_IN_ONE_KIBIBYTE as f64;
            return format!("{:} KiB", bytes_to_display);
        }

        // if it's less than a gibibyte, return it as mebibyte
        if self.bytes < Bytes::BYTES_IN_ONE_GIBIBYTE {
            let bytes_to_display = self.bytes as f64 / Byte::BYTES_IN_ONE_MEBIBYTE as f64;
            return format!("{:} MiB", bytes_to_display);
        }

        // if it's less than a tebibyte, return it as gibibyte
        if self.bytes < Byte::BYTES_IN_ONE_TEBIBYTE {
            let bytes_to_display = self.bytes as f64 / Byte::BYTES_IN_ONE_GIBIBYTE as f64;
            return format!("{:} GiB", bytes_to_display);
        }

        // if we reach this step, we have to panic since it's not supported yet
        panic!("Values larger than 1.099.511.627.775 bytes are currently not supported");
    }
}

/// The structure for representing a specific number of bytes.
pub type Bytes = Byte;

impl From<u64> for Byte {
    /// Get a [`Byte`] representation from an [`u64`].
    ///
    /// # Example
    /// ```
    /// use memory_size_type::Byte;
    ///
    /// let some_bytes = Byte::from(500);
    /// ```
    fn from(value: u64) -> Self {
        Byte { bytes: value }
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Byte {
    /// Formats the represented [`Byte`] value using the given formatter.
    ///
    /// # Example
    /// ```
    /// use memory_size_type::Byte;
    ///
    /// let one_byte = Byte::from(1);
    /// let several_bytes = Byte::from(200);
    /// let several_kibytes = Byte::from(3000);
    /// let several_odd_kibytes = Byte::from(3252);
    ///
    /// assert_eq!("1 B", format!("{}", one_byte));
    /// assert_eq!("200 B", format!("{}", several_bytes));
    /// assert_eq!("3 KiB", format!("{}", several_kibytes));
    /// assert_eq!("3.252 KiB", format!("{}", several_odd_kibytes));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_string_representation())
    }
}

#[cfg(feature = "std")]
impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_string_representation())
    }
}

#[cfg(test)]
mod tests {
    use crate::Byte;

    #[test]
    #[cfg(feature = "std")]
    fn printing_bytes_works_correctly() {
        let bytes_lower_limit = Byte::from(0);
        let bytes_middle = Byte::from(500);
        let bytes_upper_limit = Byte::from(999);

        assert_eq!(bytes_lower_limit.to_string(), "0 B");
        assert_eq!(bytes_middle.to_string(), "500 B");
        assert_eq!(bytes_upper_limit.to_string(), "999 B");
    }

    #[test]
    #[cfg(feature = "std")]
    fn printing_kibibytes_works_correctly() {
        let kbytes_lower_limit = Byte::from(1_000);
        let kbytes_middle = Byte::from(500_000);
        let kbytes_upper_limit = Byte::from(999_999);

        assert_eq!(kbytes_lower_limit.to_string(), "1 KiB");
        assert_eq!(kbytes_middle.to_string(), "500 KiB");
        assert_eq!(kbytes_upper_limit.to_string(), "999.999 KiB");
    }

    #[test]
    #[cfg(feature = "std")]
    fn printing_mebibytes_works_correctly() {
        let mbytes_lower_limit = Byte::from(1_000_000);
        let mbytes_middle = Byte::from(500_000_000);
        let mbytes_upper_limit = Byte::from(999_999_999);

        assert_eq!(mbytes_lower_limit.to_string(), "1 MiB");
        assert_eq!(mbytes_middle.to_string(), "500 MiB");
        assert_eq!(mbytes_upper_limit.to_string(), "999.999999 MiB");
    }

    #[test]
    #[cfg(feature = "std")]
    fn printing_gibibytes_works_correctly() {
        let gbytes_lower_limit = Byte::from(1_000_000_000);
        let gbytes_middle = Byte::from(500_000_000_000);
        let gbytes_upper_limit = Byte::from(999_999_999_999);

        assert_eq!(gbytes_lower_limit.to_string(), "1 GiB");
        assert_eq!(gbytes_middle.to_string(), "500 GiB");
        assert_eq!(gbytes_upper_limit.to_string(), "999.999999999 GiB");
    }
}
