use {
    crate::models::smol_cow::SmolCow,
    reqwest::RequestBuilder,
    serde::{Serialize, Serializer},
    std::fmt,
};

pub mod issue;

mod key {
    pub(super) const JQL: &'static str = "jql";
    pub(super) const START_AT: &'static str = "startAt";
    pub(super) const MAX_RESULTS: &'static str = "maxResults";
    pub(super) const VALIDATE_QUERY: &'static str = "validateQuery";
    pub(super) const WITH_FIELDS: &'static str = "fields";
    pub(super) const EXPAND: &'static str = "expand";
    pub(super) const PROPERTIES: &'static str = "properties";
    pub(super) const FIELDS_BY_KEY: &'static str = "fieldsByKeys";
    pub(super) const UPDATE_HISTORY: &'static str = "updateHistory";
    pub(super) const PROJECT_IDS: &'static str = "projectIds";
    pub(super) const PROJECT_KEYS: &'static str = "projectKeys";
    pub(super) const ISSUETYPE_IDS: &'static str = "issuetypeIds";
    pub(super) const ISSUETYPE_KEYS: &'static str = "issuetypeNames";
}

pub(crate) trait ToQuery<'a> {
    type Queries: Iterator<Item = (&'a str, OptionSerialize<'a>)>;

    fn to_queries(&'a self) -> Self::Queries;

    fn append_request(&'a self, request: RequestBuilder) -> RequestBuilder {
        self.to_queries()
            .fold(request, |req, query| req.query(&[query]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
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

#[derive(Debug)]
pub(crate) struct CommaDelimited<'a> {
    inner: &'a [SmolCow<'a, str>],
}

impl<'a> CommaDelimited<'a> {
    fn new(inner: &'a [SmolCow<'a, str>]) -> Self {
        Self { inner }
    }
}

impl<'a> Serialize for CommaDelimited<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let delimited = self.inner.join(",");
        serializer.serialize_str(delimited.as_str())
    }
}

#[derive(Debug)]
enum CommaItem<'a> {
    Text(SmolCow<'a, str>),
    Number(u64),
}

impl<'a> Serialize for CommaItem<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Text(v) => serializer.serialize_str(v.as_ref()),
            Self::Number(v) => serializer.serialize_u64(*v),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub(crate) enum OptionSerialize<'a> {
    CommaDelimited(CommaDelimited<'a>),
    Unsigned(u64),
    Bool(bool),
    Text(SmolCow<'a, str>),
    Validate(ValidateQuery),
}

impl From<ValidateQuery> for OptionSerialize<'_> {
    fn from(v: ValidateQuery) -> Self {
        Self::Validate(v)
    }
}

impl<'a> From<CommaDelimited<'a>> for OptionSerialize<'a> {
    fn from(v: CommaDelimited<'a>) -> Self {
        Self::CommaDelimited(v)
    }
}

impl<'a> From<SmolCow<'a, str>> for OptionSerialize<'a> {
    fn from(v: SmolCow<'a, str>) -> Self {
        Self::Text(v)
    }
}

impl<'a> From<&'a str> for OptionSerialize<'a> {
    fn from(v: &'a str) -> Self {
        Self::Text(v.into())
    }
}

impl From<bool> for OptionSerialize<'_> {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<usize> for OptionSerialize<'_> {
    fn from(v: usize) -> Self {
        Self::Unsigned(v as u64)
    }
}

impl From<u8> for OptionSerialize<'_> {
    fn from(v: u8) -> Self {
        Self::Unsigned(v as u64)
    }
}

impl From<u16> for OptionSerialize<'_> {
    fn from(v: u16) -> Self {
        Self::Unsigned(v as u64)
    }
}

impl From<u32> for OptionSerialize<'_> {
    fn from(v: u32) -> Self {
        Self::Unsigned(v as u64)
    }
}

impl From<u64> for OptionSerialize<'_> {
    fn from(v: u64) -> Self {
        Self::Unsigned(v)
    }
}
