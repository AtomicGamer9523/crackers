use super::*;

/// A transformer that uses the sha256 algorithm.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Sha1Transformer;

impl Transformer<20> for Sha1Transformer {
    #[inline]
    fn transform(&self, input: &[u8; 20], output: &mut [u8; 20]) {
        output.copy_from_slice(&<::sha1::Sha1 as Digest>::digest(input));
    }
}
