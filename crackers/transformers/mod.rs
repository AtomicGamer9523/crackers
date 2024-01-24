use ::digest::Digest;

pub trait Transformer<const N: usize>: Send + Sync {
    fn transform(&self, input: &[u8; N], output: &mut [u8; N]);
}

impl<const N: usize, F> Transformer<N> for F where
    F: Fn(&[u8; N], &mut [u8; N]) + Send + Sync
{
    #[inline(always)]
    fn transform(&self, input: &[u8; N], output: &mut [u8; N]) {
        self(input, output)
    }
}

pub struct Sha256Transformer;

impl Transformer<32> for Sha256Transformer {
    #[inline]
    fn transform(&self, input: &[u8; 32], output: &mut [u8; 32]) {
        output.copy_from_slice(&<sha2::Sha256 as Digest>::digest(input));
    }
}
