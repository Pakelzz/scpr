use std::{fmt::{Display}, num::ParseIntError};

#[derive(Debug)]
pub enum MyError {
    Reqwest(reqwest::Error),
    ParseJson(serde_json::Error),
    ParseInt(ParseIntError),
    TomlSer(toml::ser::Error),
    TomlDe(toml::de::Error),
    DateFormat(time::error::Parse),
    DateComponent(time::error::ComponentRange),
    DateErr(DateError),
    OutOfRange,
    ConfigNotFound,
    ConfigOutOfDate,
    Io(std::io::Error),
}

#[derive(Debug)]
pub enum DateError {
    InvalidFormat,
    InvalidMonth,
}

impl Display for DateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "Invalid date format. Expected YYYY-MM or YYYY-MM-DD"),
            Self::InvalidMonth => write!(f, "Month must between 1 and 12")
        }
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(e) => write!(f, "Request Error: {}", e),
            Self::ParseJson(e) => write!(f, "Json parse error: {}", e),
            Self::ParseInt(_e) => write!(f, "Please insert a valid number"),
            Self::TomlSer(e) => write!(f, "Toml serialize error: {}", e),
            Self::TomlDe(e) => write!(f, "Toml deserialize error: {e}"),
            Self::DateFormat(e) => write!(f, "Date parse error: {e}"),
            Self::DateErr(e) => write!(f, "{e}"),
            Self::DateComponent(e) => write!(f, "Date component error: {e}"),
            Self::OutOfRange => write!(f, "Error: The number you given is out of range"),
            Self::ConfigNotFound => write!(f, "Config Error: Config not found"),
            Self::ConfigOutOfDate => write!(f, "Config Error: Config out of date"),
            Self::Io(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for DateError {}

impl std::error::Error for MyError {}

impl From<reqwest::Error> for MyError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<serde_json::Error> for MyError {
    fn from(value: serde_json::Error) -> Self {
        Self::ParseJson(value)
    }
}

impl From<ParseIntError> for MyError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<toml::ser::Error> for MyError {
    fn from(value: toml::ser::Error) -> Self {
        Self::TomlSer(value)
    }
}

impl From<toml::de::Error> for MyError {
    fn from(value: toml::de::Error) -> Self {
        Self::TomlDe(value)
    }
}

impl From<time::error::Parse> for MyError {
    fn from(value: time::error::Parse) -> Self {
        Self::DateFormat(value)
    }
}

impl From<time::error::ComponentRange> for MyError {
    fn from(value: time::error::ComponentRange) -> Self {
        Self::DateComponent(value)
    }
}
