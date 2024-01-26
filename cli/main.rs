use clap::builder::PossibleValuesParser;
use clap::builder::TypedValueParser;
use clap::Parser;

use crackers::*;

mod config;

#[derive(Debug, Parser)]
#[command(name = "cracker")]
#[command(about = "For all your munching and cracking needs ;)", long_about = None)]
pub struct Cli {
    /// The log level to use.
    #[arg(
        short = 'l',
        value_name = "LOG_LEVEL",
        long = "log-level",
        default_value = "info",
        value_parser = PossibleValuesParser::new(config::LogLevel_PossibleValues)
            .map(|s| s.parse::<config::LogLevel>().unwrap()),
    )]
    pub log: config::LogLevel,
    /// Whether to pretty print the output.
    #[arg(
        short = 'p',
        long = "pretty",
        default_value_t = true,
    )]
    pub pretty: bool,
    /// The transformer to use.
    #[arg(
        short = 't',
        value_name = "TRANSFORMER_TYPE",
        long = "transformer",
        default_value = "sha256",
        value_parser = PossibleValuesParser::new(config::Transformers_PossibleValues)
            .map(|s| s.parse::<config::Transformers>().unwrap()),
        required = true,
    )]
    pub transformer: config::Transformers,
    /// The validator to use.
    #[command(subcommand)]
    pub validator: config::Validator,
    /// The number of threads to use for cracking.
    #[arg(
        short = 'j',
        value_name = "THREADS",
        long = "threads",
        default_value = "1"
    )]
    pub threads: u8,
    /// Whether to stop after the first solution was found.
    #[arg(
        short = 'n',
        long = "no-stop",
        default_value_t = false,
    )]
    pub no_stop: bool,
    /// Whether to only allow ascii characters as the solution.
    #[arg(
        short = 'a',
        long = "only-ascii",
        default_value_t = false,
    )]
    pub only_ascii: bool,
}

/// Minimal example.
fn main() {
    let args = Cli::parse();

    // init logging
    if args.log.not_off() {
        if let Err(e) = simple_logger::SimpleLogger::new()
            .with_level(args.log.into())
            .init() {
            eprintln!("Failed to init logger: {}", e);
        }
    }

    let validator: Box<dyn Validator> = match args.validator {
        config::Validator::StartsWith { start } => {
            Box::new(StartsWithValidator::from(start))
        },
        config::Validator::EndsWith { end } => {
            Box::new(EndsWithValidator::from(end))
        }
    };

    let transformer: Box<dyn Transformer> = match args.transformer {
        config::Transformers::Sha256 => {
            Box::new(Sha256Transformer::default())
        },
        config::Transformers::Sha1 => {
            Box::new(Sha1Transformer::default())
        }
    };

    let mut config = config((
        transformer,
        validator
    ));

    config.pretty = args.pretty;
    config.no_stop = args.no_stop;
    config.only_ascii = args.only_ascii;

    // the actual cracking
    let start = std::time::Instant::now();
    log::info!("Started cracking...");

    let input = if args.threads > 1 {
        crack_multithreaded(config, args.threads).to_vec()
    } else {
        crack(config).to_vec()
    };
    let end = std::time::Instant::now();
    let duration = end - start;
    log::info!("solution: {}, took: {:?}", String::from_utf8_lossy(&input), duration);
}