use {
    cdelim::{CommaDelimited, Element},
    serde::{Serialize, Serializer},
    std::iter::IntoIterator,
};

mod cdelim;
pub mod issue;

mod key {
    pub(super) const JQL: &str = "jql";
    pub(super) const START_AT: &str = "startAt";
    pub(super) const MAX_RESULTS: &str = "maxResults";
    pub(super) const VALIDATE_QUERY: &str = "validateQuery";
    pub(super) const WITH_FIELDS: &str = "fields";
    pub(super) const EXPAND: &str = "expand";
    pub(super) const PROPERTIES: &str = "properties";
    pub(super) const FIELDS_BY_KEY: &str = "fieldsByKeys";
    pub(super) const UPDATE_HISTORY: &str = "updateHistory";
    pub(super) const PROJECT_IDS: &str = "projectIds";
    pub(super) const PROJECT_KEYS: &str = "projectKeys";
    pub(super) const ISSUETYPE_IDS: &str = "issuetypeIds";
    pub(super) const ISSUETYPE_KEYS: &str = "issuetypeNames";
}

fn none_or_empty(o: &Option<CommaDelimited>) -> bool {
    match o {
        Some(c) => c.is_empty(),
        None => true,
    }
}
