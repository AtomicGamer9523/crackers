#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes<const N: usize>([u8; N]);

impl<const N: usize> Bytes<N> {
    #[inline(always)]
    pub const fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }
    #[inline(always)]
    pub const fn empty() -> Self {
        Self::new([0u8; N])
    }
    pub fn increment(&mut self) {
        for i in (0..N).rev() {
            if self.0[i] == u8::MAX {
                self.0[i] = 0;
            } else {
                self.0[i] += 1;
                break;
            }
        }
    }
}

impl<const N: usize> core::ops::Deref for Bytes<N> {
    type Target = [u8; N];
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
