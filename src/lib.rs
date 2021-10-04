//! A data type for dealing with memory sizes
#![doc(html_root_url = "https://docs.rs/memory-size-type/latest")]

pub struct MemorySize {
    size: u64,
}

impl MemorySize {
    ///The number of bytes in a kilobyte.
    pub const BYTES_PER_KILOBYTE: u64 = 1024;

    ///The number of bytes in a megabyte.
    pub const BYTES_PER_MEGABYTE: u64 = MemorySize::BYTES_PER_KILOBYTE * 1024;

    ///The number of bytes in a gigabyte.
    pub const BYTES_PER_GIGABYTE: u64 = MemorySize::BYTES_PER_MEGABYTE * 1024;

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
    /// assert_eq!(13, size.as_kilobytes());
    /// ```
    #[inline]
    pub const fn from_bytes(bytes: u64) -> MemorySize {
        MemorySize { size: bytes }
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
    /// assert_eq!(13312, size.as_bytes());
    /// assert_eq!(13, size.as_kilobytes());
    /// ```
    #[inline]
    pub const fn from_kilobytes(megabytes: u64) -> MemorySize {
        MemorySize {
            size: megabytes * MemorySize::BYTES_PER_KILOBYTE,
        }
    }

    /// Creates a new `MemorySize` from the specified number of whole megabytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_megabytes(13);
    ///
    /// assert_eq!(13631488, size.as_bytes());
    /// assert_eq!(13312, size.as_kilobytes());
    /// assert_eq!(13, size.as_megabytes());
    /// ```
    #[inline]
    pub const fn from_megabytes(megabytes: u64) -> MemorySize {
        MemorySize {
            size: megabytes * MemorySize::BYTES_PER_MEGABYTE,
        }
    }

    /// Creates a new `MemorySize` from the specified number of whole gigabytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::from_gigabytes(13);
    ///
    /// assert_eq!(13958643712, size.as_bytes());
    /// assert_eq!(13631488, size.as_kilobytes());
    /// assert_eq!(13312, size.as_megabytes());
    /// assert_eq!(13, size.as_gigabytes());
    /// ```
    #[inline]
    pub const fn from_gigabytes(megabytes: u64) -> MemorySize {
        MemorySize {
            size: megabytes * MemorySize::BYTES_PER_GIGABYTE,
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

    /// Returns the total number of whole kilobytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::new(13312);
    /// assert_eq!(13, size.as_kilobytes());
    /// ```
    #[inline]
    pub fn as_kilobytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_KILOBYTE)
    }

    /// Returns the total number of whole megabytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::new(13631488);
    /// assert_eq!(13, size.as_megabytes());
    /// ```
    #[inline]
    pub fn as_megabytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_MEGABYTE)
    }

    /// Returns the total number of whole gigabytes contained by this `MemorySize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use memory_size_type::MemorySize;
    ///
    /// let size = MemorySize::new(13958643712);
    /// assert_eq!(13, size.as_gigabytes());
    /// ```
    #[inline]
    pub fn as_gigabytes(&self) -> u64 {
        use num_integer::Integer;
        self.size.div_floor(&MemorySize::BYTES_PER_GIGABYTE)
    }
}
