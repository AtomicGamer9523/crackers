//! Crackers
//! 
//! For all your munching and cracking needs ;)
//! 
//! See [`Cracker`] for examples.
//! 
//! [`Cracker`]: crate::Cracker

mod transformers;
mod validators;
mod config;
mod bytes;

pub use transformers::*;
pub use validators::*;

pub use config::CrackerConfig;
pub use bytes::Bytes;

pub fn crack<T, V>(config: CrackerConfig<T, V>) -> Bytes where
    T: Transformer,
    V: Validator,
{
    let mut bytes = Bytes::new();
    bytes.push(0);

    loop {
        let transformed = config.transformer.transform(&bytes);
        if config.validator.validate(&transformed) {
            return transformed;
        }
        bytes.bitincrement();
    }
}
