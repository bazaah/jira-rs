mod create;
mod get;
mod metadata;
mod search;

use super::*;

pub use {create::*, get::*, metadata::*, search::*};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValidateQuery {
    Strict,
    Warn,
    None,
}

impl ValidateQuery {
    pub fn try_new(input: &str) -> Option<Self> {
        match input {
            "strict" => Some(Self::Strict),
            "warn" => Some(Self::Warn),
            "none" => Some(Self::None),
            _ => None,
        }
    }
}

impl Default for ValidateQuery {
    fn default() -> Self {
        Self::Strict
    }
}

impl Serialize for ValidateQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let output = match self {
            Self::Strict => "strict",
            Self::Warn => "warn",
            Self::None => "none",
        };

        serializer.serialize_str(output)
    }
}
