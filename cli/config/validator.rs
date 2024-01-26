#[derive(Debug, clap::Subcommand)]
pub enum Validator {
    /// Validates that the input bytes start with the given bytes.
    #[command(
        name = "startswith",
        arg_required_else_help = true,
    )]
    StartsWith {
        /// The bytes to validate the input starts with.
        #[arg(required = true)]
        start: String
    },
    /// Validates that the input bytes end with the given bytes.
    #[command(
        name = "endswith",
        arg_required_else_help = true,
    )]
    EndsWith {
        /// The bytes to validate the input ends with.
        #[arg(required = true)]
        end: String
    },
}
