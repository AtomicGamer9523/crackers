use crate::*;

pub struct CrackerConfig<T, V> where
    T: Transformer,
    V: Validator,
{
    pub(crate) transformer: T,
    pub(crate) validator: V,
}

impl<T, V> CrackerConfig<T, V> where
    T: Transformer,
    V: Validator,
{
    #[inline]
    pub const fn new(transformer: T, validator: V) -> Self {
        Self { transformer, validator, }
    }
}
