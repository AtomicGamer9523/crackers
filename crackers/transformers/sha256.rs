use super::*;

/// A transformer that uses the sha256 algorithm.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Sha256Transformer;

impl Transformer<32> for Sha256Transformer {
    #[inline]
    fn transform(&self, input: &[u8; 32], output: &mut [u8; 32]) {
        output.copy_from_slice(&<::sha2::Sha256 as Digest>::digest(input));
    }
}
