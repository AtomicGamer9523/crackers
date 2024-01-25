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

/// A config for cracking.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Config<const N: usize, T, V>(T, V) where
    T: Transformer<N>,
    V: Validator;

/// Creates a config.
#[inline(always)]
pub fn config<const N: usize, T, V>(
    transformer: T, validator: V
) -> Config<N, T, V> where
    T: Transformer<N>,
    V: Validator
{ Config(transformer, validator) }

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
    let transformer_ptr = r(&config.0);
    let validator_ptr = r(&config.1);

    for i in 0..threads {
        std::thread::spawn(move || {
            let mut input = Bytes::empty();
            input[N - 1] = i;
            let mut output = [0u8; N];
            let input_ptr = r(&input);
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(i as u64));
                loop {
                    log::debug!("{}", unsafe { input_ptr.gref() });
                    std::thread::sleep(std::time::Duration::from_secs(threads as u64));
                }
            });
            loop {
                transformer_ptr.transform(&input, &mut output);
    
                if llvm::unlikely(validator_ptr.validate(&output)) {
                    unsafe {
                        *done_ptr.gmut() = true;
                        *result_ptr.gmut() = Some(input);
                    }
                    log::debug!("Found the solution: {:?}", input);
                    return;
                }
    
                input.increment();
    
                if llvm::unlikely(done_ptr == true) {
                    log::trace!("Someone else found the solution");
                    return;
                }
            }
        });
    };

    loop {
        if llvm::unlikely(done_ptr == true) {
            break;
        }
        std::thread::yield_now();
    }

    return result.unwrap();
}

/// Cracks with the given config.
pub fn crack<const N: usize, T, V>(config: Config<N, T, V>) -> Bytes<N> where
    T: Transformer<N>,
    V: Validator
{
    let mut input = Bytes::empty();
    let mut output = [0u8; N];
    loop {
        config.0.transform(&input, &mut output);

        if llvm::unlikely(config.1.validate(&output)) {
            break;
        }

        input.increment();

        if llvm::unlikely(input == Bytes::empty()) {
            panic!("no valid input found");
        }
    }
    return input;
}
