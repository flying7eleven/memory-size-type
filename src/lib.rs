//! # Memory Size Type
//!
//! This crate implements a data type (`MemorySize`) for strongly typed memory size indications.
//!
//! There is support for memory units with a base of 10 (as recommended by the International
//! Electrotechnical Commission). A support for memory units with the base of 2 (as standardized
//! by IEC 80000-13) will follow soon.
#![doc(html_root_url = "https://docs.rs/memory-size-type/latest")]
#![cfg_attr(not(feature = "std"), no_std)]

pub struct MemorySize {
    size: u64,
}

impl MemorySize {
    ///The number of bytes in a kibibyte.
    const BYTES_PER_KIBIBYTE: u64 = 1024;

    ///The number of bytes in a mebibyte.
    const BYTES_PER_MEBIBYTE: u64 = MemorySize::BYTES_PER_KIBIBYTE * 1024;

    ///The number of bytes in a gibibyte.
    const BYTES_PER_GIBIBYTE: u64 = MemorySize::BYTES_PER_MEBIBYTE * 1024;

    /// Creates a new `MemorySize` from the specified number of whole bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let ten_bytes = MemorySize::new(10);
    /// ```
    #[inline]
    pub const fn new(bytes: u64) -> MemorySize {
        MemorySize { size: bytes }
    }

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

    /// Returns the total number of bytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::new(13312);
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
    /// let size = MemorySize::new(13312);
    /// assert_eq!(13, size.as_kibibytes());
    /// ```
    #[inline]
    pub fn as_kibibytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_KIBIBYTE)
    }

    /// Returns the total number of whole mebibytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::new(13631488);
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
    /// let size = MemorySize::new(13958643712);
    /// assert_eq!(13, size.as_gibibytes());
    /// ```
    #[inline]
    pub fn as_gibibytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_GIBIBYTE)
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
/// let size = MemorySize::new(13958643712);
/// assert_eq!(format!("{:?}", size), "13958643712 bytes");
/// ```
#[cfg(feature = "std")]
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
/// let size = MemorySize::new(13958643712);
/// assert_eq!(format!("{:?}", size), "13958643712 bytes");
/// ```
#[cfg(feature = "std")]
impl std::fmt::Debug for MemorySize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bytes", self.size)
    }
}
