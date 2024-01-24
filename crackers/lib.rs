//! Crackers
//! 
//! For all your munching and cracking needs ;)

mod transformers;
mod validators;
mod bytes;

pub use transformers::*;
pub use validators::*;
pub use bytes::Bytes;

pub fn crack<
    const N: usize,
    T: Transformer<N>,
    V: Validator
>(
    transformer: T,
    validator: V
) -> Bytes<N> {
    let mut input = Bytes::empty();
    let mut output = [0u8; N];
    loop {
        transformer.transform(&input, &mut output);

        if llvm::unlikely(validator.validate(&output)) {
            break;
        }

        input.increment();

        if llvm::unlikely(input == Bytes::empty()) {
            panic!("no valid input found");
        }
    }
    return input;
}
