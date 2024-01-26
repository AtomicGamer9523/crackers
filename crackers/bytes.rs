/// A wrapper around a byte array that allows for incrementing the bytes.
/// 
/// This is used internally for cracking.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes {
    increment: u8,
    bytes: Vec<u8>
}

impl Bytes {
    /// Creates a new `Bytes` with the given bytes and increment.
    #[inline(always)]
    pub const fn new_with_increment(bytes: Vec<u8>, increment: u8) -> Self {
        Self { bytes, increment }
    }
    /// Creates a new `Bytes` with the given increment.
    #[inline(always)]
    pub const fn empty_with_increment(increment: u8) -> Self {
        Self::new_with_increment(Vec::new(), increment)
    }
    /// Creates a new `Bytes` with the given bytes and an increment of 1.
    #[inline(always)]
    pub const fn new(bytes: Vec<u8>) -> Self {
        Self::new_with_increment(bytes, 1)
    }
    /// Creates a new `Bytes` with an increment of 1.
    #[inline(always)]
    pub const fn empty() -> Self {
        Self::empty_with_increment(1)
    }
    #[inline(always)]
    const fn __max(&self) -> u8 {
        u8::MAX - self.increment
    }
    /// Increments the bytes.
    pub fn increment(&mut self) {
        for i in (0..self.len()).rev() {
            if self.bytes[i] == self.__max() {
                self.bytes[i] = 0;
            } else {
                self.bytes[i] += self.increment;
                break;
            }
        }
    }
    /// Copies bytes from one place to another.
    #[inline(always)]
    pub fn copy_from(&mut self, bytes: &[u8]) {
        self.bytes.copy_from_slice(bytes);
    }
    /// Turns the bytes into a string (lossy).
    #[inline]
    pub fn string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.bytes).into()
    }
    /// Turns the bytes into a string.
    #[inline]
    pub fn string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.bytes.to_vec())
    }
}

impl AsRef<[u8]> for Bytes {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl core::ops::Deref for Bytes {
    type Target = Vec<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl core::ops::DerefMut for Bytes {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl core::fmt::Display for Bytes {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for i in 0..self.len() {
            write!(f, "{:3} ", self.bytes[i])?;
        }
        Ok(())
    }
}

impl From<Bytes> for Vec<u8> {
    #[inline(always)]
    fn from(bytes: Bytes) -> Self {
        bytes.bytes
    }
}

impl From<Vec<u8>> for Bytes {
    #[inline(always)]
    fn from(bytes: Vec<u8>) -> Self {
        Self::new(bytes)
    }
}

impl TryFrom<Bytes> for String {
    type Error = std::string::FromUtf8Error;
    #[inline(always)]
    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        bytes.string()
    }
}
