use {
    super::*,
    json::{value::RawValue as RawJson, Error as JsonError},
    serde::Serializer,
    serde_json as json,
};

#[derive(Debug, Deserialize)]
#[serde(try_from = "Box<RawJson>")]
pub struct SearchHandle {
    inner: handle::SearchInner,
}

impl SearchHandle {
    /// Try instantiate a new handle with the given backing JSON
    pub fn try_new(store: Box<RawJson>) -> Result<Self, JsonError> {
        let inner = handle::SearchInner::try_new(store, |raw| json::from_str(raw.get()))?;

        Ok(Self { inner })
    }

    /// Access this handle's data
    pub fn data(&self) -> &Search {
        self.inner.borrow_handle()
    }

    /// Consume the handle returning the backing
    /// storage
    pub fn into_inner(self) -> Box<RawJson> {
        self.inner.into_heads().store
    }
}

impl TryFrom<Box<RawJson>> for SearchHandle {
    type Error = JsonError;

    fn try_from(value: Box<RawJson>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

// Delegate the serializer to the internal handle
impl Serialize for SearchHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data().serialize(serializer)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Search<'a> {
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub expand: Cow<'a, str>,
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    #[serde(rename = "startAt")]
    pub start_at: u64,
    pub total: u64,
    pub issues: Vec<Issue<'a>>,
}

mod handle {
    use super::*;
    use ouroboros::self_referencing as ouroboros;

    #[ouroboros(pub_extras)]
    #[derive(Debug)]
    pub(super) struct SearchInner {
        store: Box<RawJson>,
        #[borrows(store)]
        pub(super) handle: Search<'this>,
    }
}

#[cfg(test)]
pub(crate) mod types {
    use crate::models::issue::issue;
    use serde_json::{json, Value as Json};

    pub fn search() -> Json {
        json!({
            "expand": "foo",
            "maxResults": 42,
            "startAt": 42,
            "total": 42,
            "issues": [issue::types::issue()]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as Json;

    #[test]
    fn deserialize_search_handle() {
        let json = jbytes(types::search());

        let search: Result<SearchHandle, _> = deserialize(&json);

        assert!(search.is_ok())
    }

    #[test]
    fn deserialize_search() {
        let json = jbytes(types::search());

        let search: Result<Search, _> = deserialize(&json);

        assert!(search.is_ok())
    }

    fn jbytes(json: Json) -> Vec<u8> {
        serde_json::to_vec(&json)
            .expect("Failed to serialize in models/issue/search tests... this is a bug")
    }

    fn deserialize<'de, 'a: 'de, T>(bytes: &'a [u8]) -> Result<T, serde_json::Error>
    where
        T: Deserialize<'de>,
    {
        let value = serde_json::from_slice(bytes).map_err(|error| {
            dbg!(&error);
            error
        });

        value
    }
}
