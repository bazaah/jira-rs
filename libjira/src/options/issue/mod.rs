mod create;
mod get;
mod metadata;
mod search;

use super::*;

pub use {create::*, get::*, metadata::*, search::*};

/// Validation level for JQL statements passed
/// to the Jira instance.
///
/// - Strict: the default level, any errors in the JQL statement
///   automatically fail the associated request with a 400.
/// - Warn: Any errors are returned as warnings, and the request
///   may "succeed" -- typically by returning nothing
/// - None: No validation is done
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
