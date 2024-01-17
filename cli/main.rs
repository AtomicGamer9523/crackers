use crackers::*;

/// Minimal example.
fn main() {
    // simple_logger::init().unwrap();

    // let mut validator = MultiValidator::new();
    // validator.push(StartsWithValidator::from([0, 0]));
    // validator.push(EndsWithValidator::from([0, 0, 0]));

    let config = CrackerConfig::new_with_perf(
        Sha256Transformer::new(),
        EndsWithValidator::from([0, 0, 0, 0]),
        CrackerPerformanceConfig::new(10),
    );

    // let start = std::time::Instant::now();
    // let bytes = crack(config);
    // let end = std::time::Instant::now();
    // log::info!("cracked: {:?} in {:?}", bytes, end - start);
    crack(config);
}