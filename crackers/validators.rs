pub trait Validator {
    fn validate(&self, bytes: &[u8]) -> bool;
}

pub struct MultiValidator(Vec<Box<dyn Validator>>);

impl MultiValidator {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn push<T: Validator + 'static>(&mut self, validator: T) {
        self.0.push(Box::new(validator));
    }
}

impl Validator for MultiValidator {
    #[inline]
    fn validate(&self, bytes: &[u8]) -> bool {
        nudge::unlikely(self.0.iter().all(|v| v.validate(bytes)))
    }
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
        nudge::unlikely(bytes.starts_with(&self.0))
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
        nudge::unlikely(bytes.ends_with(&self.0))
    }
}
