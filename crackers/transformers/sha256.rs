use super::*;

/// A transformer that uses the sha256 algorithm.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Sha256Transformer;

impl Transformer for Sha256Transformer {
    #[inline]
    fn transform(&self, input: &Bytes, output: &mut Bytes) {
        output.copy_from_slice(&<::sha2::Sha256 as Digest>::digest(input));
    }
    #[inline(always)]
    fn init_bytes(&self) -> Bytes {
        let mut vec = Vec::new();
        for _ in 0..32 {
            vec.push(65);
        }
        Bytes::new(vec)
    }
}
