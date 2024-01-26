use crate::*;

/// A config for cracking.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Config<T, V> where
    T: Transformer,
    V: Validator
{
    /// The transformer to use.
    pub transformer: T,
    /// The validator to use.
    pub validator: V,
    /// Whether to use pretty logging.
    pub pretty: bool,
    /// Whether to stop after the first solution was found.
    pub no_stop: bool,
    /// Whether to only allow ascii characters as the solution.
    pub only_ascii: bool,
}

/// Something that can be converted into a config.
pub trait IntoConfig<T, V> where
    T: Transformer,
    V: Validator
{
    /// Converts into a config.
    fn into_config(self) -> Config<T, V>;
}

impl<T, V> IntoConfig<T, V> for Config<T, V> where
    T: Transformer,
    V: Validator
{
    #[inline(always)]
    fn into_config(self) -> Config<T, V> { self }
}

impl<T, V> IntoConfig<T, V> for (T, V) where
    T: Transformer,
    V: Validator
{
    #[inline(always)]
    fn into_config(self) -> Config<T, V> {
        Config {
            transformer: self.0,
            validator: self.1,
            pretty: false,
            no_stop: false,
            only_ascii: true
        }
    }
}

impl<T, V> Config<T, V> where
    T: Transformer,
    V: Validator
{
    /// Creates a config from something that can be converted into a config.
    #[inline(always)]
    pub fn from<I: IntoConfig<T, V>>(i: I) -> Self {
        i.into_config()
    }
}
