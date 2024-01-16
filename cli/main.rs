use crackers::*;

/// Minimal example.
fn main() {
    simple_logger::init().unwrap();

    let config = CrackerConfig::new(
        Sha256Transformer::new(),
        EndsWithValidator::from("000"),
    );

    let bytes = crack(config);
    log::info!("cracked: {:?}", bytes)
}