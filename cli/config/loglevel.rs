pub const LogLevel_PossibleValues: [&'static str; 18] = [
    "trace", "t", "5",
    "debug", "d", "4",
    "info",  "i", "3",
    "warn",  "w", "2",
    "error", "e", "1",
    "off",   "o", "0"
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,

    Off
}

impl LogLevel {
    pub fn not_off(&self) -> bool {
        match self {
            Self::Off => false,
            _ => true
        }
    }
}

impl From<LogLevel> for log::LevelFilter {
    fn from(l: LogLevel) -> Self {
        match l {
            LogLevel::Trace => Self::Trace,
            LogLevel::Debug => Self::Debug,
            LogLevel::Info => Self::Info,
            LogLevel::Warn => Self::Warn,
            LogLevel::Error => Self::Error,
            LogLevel::Off => Self::Off
        }
    }
}

impl core::str::FromStr for LogLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" | "t" | "5" => Ok(Self::Trace),
            "debug" | "d" | "4" => Ok(Self::Debug),
            "info"  | "i" | "3" => Ok(Self::Info),
            "warn"  | "w" | "2" => Ok(Self::Warn),
            "error" | "e" | "1" => Ok(Self::Error),
            "off"   | "o" | "0" => Ok(Self::Off),
            _ => Err(format!("Unknown log level: {}", s))
        }
    }
}
