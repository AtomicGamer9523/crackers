use crate::*;

pub struct CrackerConfig<T, V> where
    T: Transformer,
    V: Validator,
{
    pub(crate) perf: Option<CrackerPerformanceConfig>,
    pub(crate) transformer: T,
    pub(crate) validator: V,
}

unsafe impl<T, V> Send for CrackerConfig<T, V> where
    T: Transformer,
    V: Validator,
{}

unsafe impl<T, V> Sync for CrackerConfig<T, V> where
    T: Transformer,
    V: Validator,
{}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CrackerPerformanceConfig {
    pub(crate) threads: usize,
}

impl CrackerPerformanceConfig {
    #[inline]
    pub const fn new(threads: usize) -> Self {
        Self { threads, }
    }
}

impl Default for CrackerPerformanceConfig {
    #[inline]
    fn default() -> Self {
        Self::new(1)
    }
}

impl<T, V> CrackerConfig<T, V> where
    T: Transformer,
    V: Validator,
{
    #[inline]
    pub const fn new(transformer: T, validator: V) -> Self {
        Self { transformer, validator, perf: None }
    }
    pub const fn new_with_perf(transformer: T, validator: V, perf: CrackerPerformanceConfig) -> Self {
        Self { transformer, validator, perf: Some(perf) }
    }
}
