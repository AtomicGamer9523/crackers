use crackers::*;

/// Minimal example.
fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Trace)
        .init().unwrap();

    // the actual cracking
    let start = std::time::Instant::now();
    log::info!("Started cracking...");
    let config = config(
        Sha256Transformer,
        StartsWithValidator::from(&[1, 5]),
    );
    let input = crack_multithreaded(config, 12);
    // let input = crack(config);
    let end = std::time::Instant::now();
    let duration = end - start;
    log::info!("input: {}, took: {:?}", input, duration);
}