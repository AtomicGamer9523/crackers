use crate::*;

/// A config for cracking.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Config<const N: usize, T, V> where
    T: Transformer<N>,
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
pub trait IntoConfig<const N: usize, T, V> where
    T: Transformer<N>,
    V: Validator
{
    /// Converts into a config.
    fn into_config(self) -> Config<N, T, V>;
}

impl<const N: usize, T, V> IntoConfig<N, T, V> for Config<N, T, V> where
    T: Transformer<N>,
    V: Validator
{
    #[inline(always)]
    fn into_config(self) -> Config<N, T, V> { self }
}

impl<const N: usize, T, V> IntoConfig<N, T, V> for (T, V) where
    T: Transformer<N>,
    V: Validator
{
    #[inline(always)]
    fn into_config(self) -> Config<N, T, V> {
        Config {
            transformer: self.0,
            validator: self.1,
            pretty: false,
            no_stop: false,
            only_ascii: true
        }
    }
}

impl<const N: usize, T, V> Config<N, T, V> where
    T: Transformer<N>,
    V: Validator
{
    /// Creates a config from something that can be converted into a config.
    #[inline(always)]
    pub fn from<I: IntoConfig<N, T, V>>(i: I) -> Self {
        i.into_config()
    }
}
