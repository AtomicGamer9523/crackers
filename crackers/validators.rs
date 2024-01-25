/// Validators are used to check if the the output of a generator
/// is what we are looking for.
pub trait Validator: Send + Sync {
    /// Returns true if the given bytes are valid.
    fn validate(&self, bytes: &[u8]) -> bool;
}

/// A validator that checks if the given bytes start with the given bytes.
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

/// A validator that checks if the given bytes end with the given bytes.
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
