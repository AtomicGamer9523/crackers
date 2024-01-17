#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes {
    vec: Vec<u8>,
    increment: u8,
    start: u8,
}

impl core::fmt::UpperHex for Bytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.iter() {
            write!(f, "0x{:02X} ", byte)?;
        }
        Ok(())
    }
}

impl core::fmt::LowerHex for Bytes {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.iter() {
            write!(f, "0x{:02x} ", byte)?;
        }
        Ok(())
    }
}

impl Bytes {
    pub const fn new() -> Self {
        Self { vec: Vec::new(), increment: 1, start: 0 }
    }

    pub const fn new_with_increment(increment: u8) -> Self {
        Self { vec: Vec::new(), increment, start: 0 }
    }

    pub fn new_with_start(start: u8) -> Self {
        Self { vec: vec![start], increment: 1, start }
    }

    pub fn new_with_start_and_increment(start: u8, increment: u8) -> Self {
        Self { vec: vec![start], increment, start }
    }

    pub fn bitincrement(&mut self) {
        let max: u8 = 255 - self.increment;
        let inc = self.increment;
        let strt = self.start;
        let mut carry = true;
        for byte in self.iter_mut() {
            if carry {
                if *byte >= max {
                    *byte = strt;
                    carry = true;
                } else {
                    *byte += inc;
                    carry = false;
                }
            }
        }
        if carry {
            self.push(0);
        }
    }
}

impl core::ops::Deref for Bytes {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl core::ops::DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(input: Vec<u8>) -> Self {
        Self { vec: input, increment: 1, start: 0 }
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(input: Bytes) -> Self {
        input.vec
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.vec.as_ref()
    }
}
