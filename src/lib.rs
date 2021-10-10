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
    type Output = u64;

    /// Performs the `+` operation on a byte.
    ///
    /// # Example
    ///
    /// ```
    /// use memory_size_type::Byte;
    ///
    /// let some_bytes = Byte::from(256);
    /// let some_more_bytes = Byte::from(256);
    /// assert_eq!(some_bytes + some_more_bytes, 512);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0
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

#[cfg(feature = "deprecated")]
pub struct MemorySize {
    size: u64,
}

#[cfg(feature = "deprecated")]
impl MemorySize {
    ///The number of bytes in a kibibyte.
    const BYTES_PER_KIBIBYTE: u64 = 1024;

    ///The number of bytes in a kilobyte.
    const BYTES_PER_KILOBYTE: u64 = 1000;

    ///The number of bytes in a mebibyte.
    const BYTES_PER_MEBIBYTE: u64 = MemorySize::BYTES_PER_KIBIBYTE * 1024;

    ///The number of bytes in a gibibyte.
    const BYTES_PER_GIBIBYTE: u64 = MemorySize::BYTES_PER_MEBIBYTE * 1024;

    ///The number of bytes in a tebibyte.
    const BYTES_PER_TEBIBYTE: u64 = MemorySize::BYTES_PER_GIBIBYTE * 1024;

    ///The number of bytes in a pebibyte.
    const BYTES_PER_PEBIBYTE: u64 = MemorySize::BYTES_PER_TEBIBYTE * 1024;

    /// Creates a new `MemorySize` from the specified number of whole bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(13312);
    ///
    /// assert_eq!(13312, size.as_bytes());
    /// assert_eq!(13, size.as_kibibytes());
    /// ```
    #[inline]
    pub const fn from_bytes(bytes: u64) -> MemorySize {
        MemorySize { size: bytes }
    }

    /// Creates a new `MemorySize` from the specified number of whole kibibytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_kibibytes(13);
    ///
    /// assert_eq!(13312, size.as_bytes());
    /// assert_eq!(13, size.as_kibibytes());
    /// ```
    #[inline]
    pub const fn from_kibibytes(megabytes: u64) -> MemorySize {
        MemorySize {
            size: megabytes * MemorySize::BYTES_PER_KIBIBYTE,
        }
    }

    /// Creates a new `MemorySize` from the specified number of whole kilobytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_kilobytes(13);
    ///
    /// assert_eq!(13000, size.as_bytes());
    /// assert_eq!(13, size.as_kilobytes());
    /// ```
    #[inline]
    pub const fn from_kilobytes(megabytes: u64) -> MemorySize {
        MemorySize {
            size: megabytes * MemorySize::BYTES_PER_KILOBYTE,
        }
    }

    /// Creates a new `MemorySize` from the specified number of whole mebibytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_mebibytes(13);
    ///
    /// assert_eq!(13631488, size.as_bytes());
    /// assert_eq!(13312, size.as_kibibytes());
    /// assert_eq!(13, size.as_mebibytes());
    /// ```
    #[inline]
    pub const fn from_mebibytes(megabytes: u64) -> MemorySize {
        MemorySize {
            size: megabytes * MemorySize::BYTES_PER_MEBIBYTE,
        }
    }

    /// Creates a new `MemorySize` from the specified number of whole gigibytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_gibibytes(13);
    ///
    /// assert_eq!(13958643712, size.as_bytes());
    /// assert_eq!(13631488, size.as_kibibytes());
    /// assert_eq!(13312, size.as_mebibytes());
    /// assert_eq!(13, size.as_gibibytes());
    /// ```
    #[inline]
    pub const fn from_gibibytes(megabytes: u64) -> MemorySize {
        MemorySize {
            size: megabytes * MemorySize::BYTES_PER_GIBIBYTE,
        }
    }

    /// Creates a new `MemorySize` from the specified number of whole tebibytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_tebibytes(13);
    ///
    /// assert_eq!(14293651161088, size.as_bytes());
    /// assert_eq!(13958643712, size.as_kibibytes());
    /// assert_eq!(13631488, size.as_mebibytes());
    /// assert_eq!(13312, size.as_gibibytes());
    /// assert_eq!(13, size.as_tebibytes());
    /// ```
    #[inline]
    pub const fn from_tebibytes(megabytes: u64) -> MemorySize {
        MemorySize {
            size: megabytes * MemorySize::BYTES_PER_TEBIBYTE,
        }
    }

    /// Creates a new `MemorySize` from the specified number of whole pebibytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_pebibytes(13);
    ///
    /// assert_eq!(14636698788954112, size.as_bytes());
    /// assert_eq!(14293651161088, size.as_kibibytes());
    /// assert_eq!(13958643712, size.as_mebibytes());
    /// assert_eq!(13631488, size.as_gibibytes());
    /// assert_eq!(13312, size.as_tebibytes());
    /// assert_eq!(13, size.as_pebibytes());
    /// ```
    #[inline]
    pub const fn from_pebibytes(megabytes: u64) -> MemorySize {
        MemorySize {
            size: megabytes * MemorySize::BYTES_PER_PEBIBYTE,
        }
    }

    /// Returns the total number of bytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(13312);
    /// assert_eq!(13312, size.as_bytes());
    /// ```
    #[inline]
    pub const fn as_bytes(&self) -> u64 {
        self.size
    }

    /// Returns the total number of whole kibibytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(13312);
    /// assert_eq!(13, size.as_kibibytes());
    /// ```
    #[inline]
    pub fn as_kibibytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_KIBIBYTE)
    }

    /// Returns the total number of whole kilobyte contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(13000);
    /// assert_eq!(13, size.as_kilobytes());
    /// ```
    #[inline]
    pub fn as_kilobytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_KILOBYTE)
    }

    /// Returns the total number of whole mebibytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(13631488);
    /// assert_eq!(13, size.as_mebibytes());
    /// ```
    #[inline]
    pub fn as_mebibytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_MEBIBYTE)
    }

    /// Returns the total number of whole gibibytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(13958643712);
    /// assert_eq!(13, size.as_gibibytes());
    /// ```
    #[inline]
    pub fn as_gibibytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_GIBIBYTE)
    }

    /// Returns the total number of whole tebibytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(14293651161088);
    /// assert_eq!(13, size.as_tebibytes());
    /// ```
    #[inline]
    pub fn as_tebibytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_TEBIBYTE)
    }

    /// Returns the total number of whole pebibytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_bytes(14636698788954112);
    /// assert_eq!(13, size.as_pebibytes());
    /// ```
    #[inline]
    pub fn as_pebibytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_PEBIBYTE)
    }
}

/// The implementation for the `std::fmt::Display` trait which prints out the `MemorySize` as an
/// human-readable text in bytes.
///
/// # Example
///
/// ```
/// use memory_size_type::MemorySize;
///
/// let size = MemorySize::from_bytes(13958643712);
/// assert_eq!(format!("{}", size), "13958643712 bytes");
/// ```
#[cfg(all(feature = "std", feature = "deprecated"))]
impl std::fmt::Display for MemorySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bytes", self.size) // TODO: should be replaced by a method which use the most useful size unit
    }
}

/// The implementation for the `std::fmt::Debug` trait which prints out the `MemorySize` as an
/// human-readable text in bytes.
///
/// # Example
///
/// ```
/// use memory_size_type::MemorySize;
///
/// let size = MemorySize::from_bytes(13958643712);
/// assert_eq!(format!("{:?}", size), "13958643712 bytes");
/// ```
#[cfg(all(feature = "std", feature = "deprecated"))]
impl std::fmt::Debug for MemorySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bytes", self.size)
    }
}
