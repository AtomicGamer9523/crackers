use crate::*;

pub trait Transformer: Send + Sync {
    fn transform(&self, input: &Bytes) -> Bytes;
}

impl<F: Fn(&Bytes) -> Bytes + Send + Sync> Transformer for F {
    #[inline]
    fn transform(&self, input: &Bytes) -> Bytes {
        self(input)
    }
}

pub struct Sha256Transformer(sha2::Sha256);

impl Sha256Transformer {
    #[inline]
    pub fn new() -> Self {
        Self(<sha2::Sha256 as sha2::Digest>::new())
    }
}

impl Transformer for Sha256Transformer {
    #[inline]
    fn transform(&self, input: &Bytes) -> Bytes {
        Bytes::from(<sha2::Sha256 as sha2::Digest>::digest(input).to_vec())
    }
}
