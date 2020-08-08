use {
    super::*,
    json::{value::RawValue as RawJson, Error as JsonError},
    serde::{Deserialize, Serialize},
    serde_json as json,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Search {
    pub expand: String,
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    #[serde(rename = "startAt")]
    pub start_at: u64,
    pub total: u64,
    pub issues: Vec<Issue>,
}
