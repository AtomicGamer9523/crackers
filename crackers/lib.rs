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

// use std::sync::{Mutex, Arc, atomic::AtomicBool};

pub use transformers::*;
pub use validators::*;

pub use config::*;
pub use bytes::*;

mod unsafe_magic;
use unsafe_magic::UnsafeMultithreadedRcExt;

pub fn crack<T, V>(config: CrackerConfig<T, V>) -> Bytes where
    T: Transformer + 'static,
    V: Validator + 'static,
{
    let mut thread_count: u8 = 1;

    if let Some(perf) = config.perf {
        thread_count = perf.threads as u8;
    }

    let transformer_owner = config.transformer.as_unsafe_multithreaded_rc();
    let validator_owner = config.validator.as_unsafe_multithreaded_rc();
    let transformer = transformer_owner.thread_shared_ref();
    let validator = validator_owner.thread_shared_ref();

    // let done = Arc::new(AtomicBool::new(false));
    // let res = Arc::new(Mutex::new(None));

    for i in 0..thread_count {
        // let r = res.clone();
        // let d = done.clone();
        std::thread::spawn(move || {
            let mut bytes = Bytes::new_with_start_and_increment(i, thread_count);
            loop {
                // if d.load(std::sync::atomic::Ordering::Relaxed) {
                //     break;
                // }
                if validator.validate(&transformer.transform(&bytes)) {
                    println!(
                        "{i} - cracked: {:X}",
                        &bytes,
                    );
                    // r.lock().unwrap().replace(bytes.clone());
                    std::process::exit(0);
                    // break;
                }
                bytes.bitincrement();
            }
            // d.store(true, std::sync::atomic::Ordering::Relaxed);
        });
    }

    loop {

    }
// 
//     while !done.load(std::sync::atomic::Ordering::Relaxed) {
//     }

    // res.clone()
    //     .lock()
    //     .unwrap()
    //     .take()
    //     .unwrap()
}
