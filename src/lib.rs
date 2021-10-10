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
#![allow(clippy::must_use_candidate)]

/// The structure for representing a specific number of bytes.
pub struct Byte(u64);

/// The structure for representing a specific number of bytes.
pub type Bytes = Byte;

impl From<u64> for Byte {
    /// Get a byte representation from a u64 number.
    ///
    /// # Example
    /// ```
    /// use memory_size_type::Byte;
    ///
    /// let some_bytes = Byte::from(256);
    /// ```
    fn from(value: u64) -> Self {
        Byte(value)
    }
}

#[cfg(feature = "std")]
impl std::ops::Add for Byte {
    type Output = Byte;

    /// Performs the `+` operation on a byte.
    ///
    /// # Example
    ///
    /// ```
    /// use memory_size_type::Byte;
    ///
    /// let some_bytes = Byte::from(256);
    /// let some_more_bytes = Byte::from(256);
    /// assert_eq!((some_bytes + some_more_bytes).to_string(), "512 bytes");
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Byte(self.0 + rhs.0)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Byte {
    /// Formats the represented byte value using the given formatter.
    ///
    /// # Example
    /// ```
    /// use memory_size_type::Byte;
    ///
    /// let one_byte = Byte::from(1);
    /// let several_bytes = Byte::from(256);
    ///
    /// assert_eq!("1 byte", format!("{}", one_byte));
    /// assert_eq!("256 bytes", format!("{}", several_bytes));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 1 {
            return write!(f, "{} byte", self.0);
        }
        write!(f, "{} bytes", self.0)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Debug for Byte {
    /// Formats the represented byte value using the given formatter in a programmer-facing,
    /// debugging context.
    ///
    /// # Example
    /// ```
    /// use memory_size_type::Byte;
    ///
    /// let one_byte = Byte::from(1);
    /// let several_bytes = Byte::from(256);
    ///
    /// assert_eq!("1 byte", format!("{}", one_byte));
    /// assert_eq!("256 bytes", format!("{}", several_bytes));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 1 {
            return write!(f, "{} byte", self.0);
        }
        write!(f, "{} bytes", self.0)
    }
}

/// The structure for representing a specific number of kibibytes (1 kibibyte = 1024 bytes).
pub struct Kibibyte(u64);

/// The structure for representing a specific number of kibibytes (1 kibibyte = 1024 bytes).
pub type Kibibytes = Kibibyte;

impl From<u64> for Kibibyte {
    /// Get a kibibyte representation from a u64 number representing the number of kibibytes.
    ///
    /// # Example
    /// ```
    /// use memory_size_type::Kibibyte;
    ///
    /// let some_bytes = Kibibyte::from(256);
    /// ```
    fn from(value: u64) -> Self {
        Kibibyte(value)
    }
}

impl From<Byte> for Kibibyte {
    /// Get a kibibyte representation from a Byte representation.
    ///
    /// # Example
    /// ```
    /// use memory_size_type::{Kibibyte, Byte};
    ///
    /// let some_kibibytes = Kibibyte::from(Byte::from(256));
    /// ```
    fn from(value: Byte) -> Self {
        const KIBIBYTES_PER_BYTE: u64 = 1024;

        Kibibyte(value.0 * KIBIBYTES_PER_BYTE)
    }
}

#[cfg(feature = "std")]
impl std::ops::Add for Kibibyte {
    type Output = Kibibyte;

    /// Performs the `+` operation on a byte.
    ///
    /// # Example
    ///
    /// ```
    /// use memory_size_type::Kibibyte;
    ///
    /// let some_kibibytes = Kibibyte::from(256);
    /// let some_more_kibibytes = Kibibyte::from(256);
    /// assert_eq!((some_kibibytes + some_more_kibibytes).to_string(), "512 kibibytes");
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Kibibyte(self.0 + rhs.0)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for Kibibyte {
    /// Formats the represented kibibyte value using the given formatter.
    ///
    /// # Example
    /// ```
    /// use memory_size_type::Kibibyte;
    ///
    /// let one_kibibyte = Kibibyte::from(1);
    /// let several_kibibytes = Kibibyte::from(256);
    ///
    /// assert_eq!("1 kibibyte", format!("{}", one_kibibyte));
    /// assert_eq!("256 kibibytes", format!("{}", several_kibibytes));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 1 {
            return write!(f, "{} kibibyte", self.0);
        }
        write!(f, "{} kibibytes", self.0)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Debug for Kibibyte {
    /// Formats the represented kibibyte value using the given formatter in a programmer-facing,
    /// debugging context.
    ///
    /// # Example
    /// ```
    /// use memory_size_type::Kibibyte;
    ///
    /// let one_kibibyte = Kibibyte::from(1);
    /// let several_kibibytes = Kibibyte::from(256);
    ///
    /// assert_eq!("1 kibibyte", format!("{}", one_kibibyte));
    /// assert_eq!("256 kibibytes", format!("{}", several_kibibytes));
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 1 {
            return write!(f, "{} kibibyte", self.0);
        }
        write!(f, "{} kibibytes", self.0)
    }
}
