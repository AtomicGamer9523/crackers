use ::digest::Digest;
use crate::Bytes;

/// A transformer is used to transform the input bytes into output bytes.
pub trait Transformer: Send + Sync {
    /// Transforms the input bytes into output bytes.
    /// 
    /// By default, this copies the input bytes into the output bytes.
    #[inline(always)]
    fn transform(&self, input: &Bytes, output: &mut Bytes) {
        output.copy_from_slice(input);
    }
    /// Initializes the bytes.
    /// 
    /// By default, this returns an empty `Bytes`.
    fn init_bytes(&self) -> Bytes;
}

impl<T> Transformer for Box<T> where
    T: Transformer + ?Sized
{
    /// Automatically implements the `Transformer` trait for all boxed
    /// `Transformer`s.
    #[inline(always)]
    fn transform(&self, input: &Bytes, output: &mut Bytes) {
        (**self).transform(input, output)
    }
    /// Automatically implements the `Transformer` trait for all boxed
    /// `Transformer`s.
    #[inline(always)]
    fn init_bytes(&self) -> Bytes {
        (**self).init_bytes()
    }
}

mod sha256;
mod sha1;
mod md5;

pub use self::sha256::Sha256Transformer;
pub use self::sha1::Sha1Transformer;
pub use self::md5::Md5Transformer;
