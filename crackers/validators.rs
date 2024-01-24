pub trait Validator: Send + Sync {
    fn validate(&self, bytes: &[u8]) -> bool;
}

pub struct StartsWithValidator(Vec<u8>);

impl<T: Into<Vec<u8>>> From<T> for StartsWithValidator {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Validator for StartsWithValidator {
    #[inline]
    fn validate(&self, bytes: &[u8]) -> bool {
        llvm::unlikely(bytes.starts_with(&self.0))
    }
}

pub struct EndsWithValidator(Vec<u8>);

impl<T: Into<Vec<u8>>> From<T> for EndsWithValidator {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Validator for EndsWithValidator {
    #[inline]
    fn validate(&self, bytes: &[u8]) -> bool {
        llvm::unlikely(bytes.ends_with(&self.0))
    }
}
