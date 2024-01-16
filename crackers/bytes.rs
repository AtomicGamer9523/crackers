#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn bitincrement(&mut self) {
        if nudge::unlikely(self.is_empty()) {
            self.push(0);
        }
        let mut carry = true;
        for byte in self.iter_mut().rev() {
            if carry {
                if byte == &255 {
                    *byte = 0;
                    carry = true;
                } else {
                    *byte += 1;
                    carry = false;
                }
            }
        }
        if carry {
            self.push(1);
        }
    }
}

impl core::ops::Deref for Bytes {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(input: Vec<u8>) -> Self {
        Self(input)
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(input: Bytes) -> Self {
        input.0
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
