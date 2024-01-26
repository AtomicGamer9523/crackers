use ::digest::Digest;

/// A transformer is used to transform the input bytes into output bytes.
pub trait Transformer<const N: usize>: Send + Sync {
    /// Transforms the input bytes into output bytes.
    /// 
    /// By default, this copies the input bytes into the output bytes.
    #[inline(always)]
    fn transform(&self, input: &[u8; N], output: &mut [u8; N]) {
        output.copy_from_slice(input);
    }
}

impl<const N: usize, F> Transformer<N> for F where
    F: Fn(&[u8; N], &mut [u8; N]) + Send + Sync
{
    /// Automatically implements the `Transformer` trait for all functions
    /// that take a `&[u8; N]` and a `&mut [u8; N]` and return `()`.
    #[inline(always)]
    fn transform(&self, input: &[u8; N], output: &mut [u8; N]) {
        self(input, output)
    }
}

mod sha256;
mod sha1;

pub use self::sha256::Sha256Transformer;
pub use self::sha1::Sha1Transformer;
