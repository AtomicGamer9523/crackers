pub const Transformers_PossibleValues: [&'static str; 2] = ["sha256", "sha1"];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Transformers {
    Sha256,
    Sha1
}

impl core::fmt::Display for Transformers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sha256 => write!(f, "sha256"),
            Self::Sha1 => write!(f, "sha1")
        }
    }
}

impl core::str::FromStr for Transformers {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sha256" => Ok(Self::Sha256),
            "sha1" => Ok(Self::Sha1),
            _ => Err(format!("Unknown hash type: {}", s))
        }
    }
}
