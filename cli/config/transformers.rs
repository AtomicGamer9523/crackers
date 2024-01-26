pub const Transformers_PossibleValues: [&'static str; 2] = ["sha256", "sha1"];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Transformers {
    Sha256,
    Sha1
}

impl ::crackers::Transformer<32> for Transformers {
    #[inline(always)]
    fn transform(&self, input: &[u8; 32], output: &mut [u8; 32]) {
        match self {
            Self::Sha256 => {
                ::crackers::Sha256Transformer.transform(input, output)
            },
            _ => unreachable!("sha1 is not supported for 32 bytes")
        }
    }
}

impl ::crackers::Transformer<20> for Transformers {
    #[inline(always)]
    fn transform(&self, input: &[u8; 20], output: &mut [u8; 20]) {
        match self {
            Self::Sha1 => {
                ::crackers::Sha1Transformer.transform(input, output)
            },
            _ => unreachable!("sha256 is not supported for 20 bytes")
        }
    }
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
