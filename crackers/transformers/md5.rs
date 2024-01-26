use super::*;

/// A transformer that uses the sha256 algorithm.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Md5Transformer;

impl Transformer for Md5Transformer {
    #[inline]
    fn transform(&self, input: &Bytes, output: &mut Bytes) {
        output.copy_from(&<::md5::Md5 as Digest>::digest(input));
    }
    #[inline(always)]
    fn init_bytes(&self) -> Bytes {
        let mut vec = Vec::new();
        for _ in 0..16 {
            vec.push(65);
        }
        Bytes::new(vec)
    }
}
