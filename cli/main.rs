use crackers::*;

/// Minimal example.
fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init().unwrap();

    // the actual cracking
    let input = crack(
        Sha256Transformer,
        StartsWithValidator::from(&[1, 2, 3, 4])
    );
    println!("input: {:?}", input)
}