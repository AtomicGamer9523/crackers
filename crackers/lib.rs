//! Crackers
//! 
//! For all your munching and cracking needs ;)

#![deny(unused, missing_docs)]

mod transformers;
mod validators;
mod bytes;

pub use transformers::*;
pub use validators::*;
pub use bytes::Bytes;

mod config;
pub use config::Config;

/// Prelude for the crackers crate.
pub mod prelude {
    pub use super::config::IntoConfig;
}

use prelude::IntoConfig;

/// Creates a config.
#[inline(always)]
pub fn config<const N: usize, T, V, I>(i: I) -> Config<N, T, V> where
    I: IntoConfig<N, T, V>,
    T: Transformer<N>,
    V: Validator,
{ Config::from(i) }

/// Cracks with the given config.
/// Uses multiple threads.
pub fn crack_multithreaded<const N: usize, T, V>(
    config: Config<N, T, V>, threads: u8
) -> Bytes<N> where
    T: Transformer<N> + 'static,
    V: Validator + 'static
{
    use llvm::*;

    #[allow(unused)]
    let mut result = Option::<Bytes<N>>::None;
    let result_ptr = r(&result);
    #[allow(unused)]
    let mut done = false;
    let done_ptr = r(&done);
    let transformer_ptr = r(&config.transformer);
    let validator_ptr = r(&config.validator);

    for i in 0..threads {
        std::thread::spawn(move || {
            let mut input = Bytes::new([65u8; N]);
            input[0] = 65 + i;
            let mut output = [0u8; N];
            let input_ptr = r(&input);
            if config.pretty {
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(i as u64));
                    loop {
                        log::debug!("{}", unsafe { input_ptr.gref() });
                        std::thread::sleep(std::time::Duration::from_secs(threads as u64));
                    }
                });
            }
            loop {
                transformer_ptr.transform(&input, &mut output);
    
                if llvm::unlikely(validator_ptr.validate(&output, config.only_ascii)) {
                    log::debug!("Found the solution: {:?}", input);
                    if let Some(s) = input.string().ok() {
                        log::info!("Solution: {} - {}", s, input);
                    }
                    if !config.no_stop {
                        unsafe {
                            *done_ptr.gmut() = true;
                            *result_ptr.gmut() = Some(input);
                        }
                        return;
                    }
                }
    
                input.increment();
    
                if !config.no_stop {
                    if llvm::unlikely(done_ptr == true) {
                        log::trace!("Someone else found the solution");
                        return;
                    }
                }
            }
        });
    };

    loop {
        std::thread::yield_now();
        if llvm::unlikely(done_ptr == true) {
            break;
        }
    }

    unsafe {
        return result.unwrap_unchecked();
    }
}

/// Cracks with the given config.
pub fn crack<const N: usize, T, V>(config: Config<N, T, V>) -> Bytes<N> where
    T: Transformer<N>,
    V: Validator
{
    let mut input = Bytes::new([65u8; N]);
    let mut output = [0u8; N];
    loop {
        config.transformer.transform(&input, &mut output);

        if llvm::unlikely(config.validator.validate(&output, config.only_ascii)) {
            if !config.no_stop {
                break;
            }
        }

        input.increment();
    }
    return input;
}
