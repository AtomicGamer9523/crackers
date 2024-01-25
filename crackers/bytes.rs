/// A wrapper around a byte array that allows for incrementing the bytes.
/// 
/// This is used internally for cracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes<const N: usize> {
    increment: u8,
    bytes: [u8; N]
}

impl<const N: usize> Bytes<N> {
    /// Creates a new `Bytes` with the given bytes and increment.
    #[inline(always)]
    pub const fn new_with_increment(bytes: [u8; N], increment: u8) -> Self {
        Self { bytes, increment }
    }
    /// Creates a new `Bytes` with the given increment.
    #[inline(always)]
    pub const fn empty_with_increment(increment: u8) -> Self {
        Self::new_with_increment([0u8; N], increment)
    }
    /// Creates a new `Bytes` with the given bytes and an increment of 1.
    #[inline(always)]
    pub const fn new(bytes: [u8; N]) -> Self {
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
        for i in (0..N).rev() {
            if self.bytes[i] == self.__max() {
                self.bytes[i] = 0;
            } else {
                self.bytes[i] += self.increment;
                break;
            }
        }
    }
}

impl<const N: usize> core::ops::Deref for Bytes<N> {
    type Target = [u8; N];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl<const N: usize> core::ops::DerefMut for Bytes<N> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl<const N: usize> core::fmt::Display for Bytes<N> {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for i in 0..N {
            write!(f, "{:3} ", self.bytes[i])?;
        }
        Ok(())
    }
}
