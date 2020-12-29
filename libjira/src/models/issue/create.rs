use {
    super::*,
    json::{value::RawValue as RawJson, Error as JsonError},
    serde::Serializer,
    serde_json as json,
};

#[derive(Debug, Deserialize)]
#[serde(try_from = "Box<RawJson>")]
pub struct CreatedHandle {
    // This handle must never be exposed in the public API
    inner: handle::CreatedInner,
}

impl CreatedHandle {
    /// Try instantiate a new handle with the given backing JSON
    pub fn try_new(store: Box<RawJson>) -> Result<Self, JsonError> {
        let inner = handle::CreatedInner::try_new(store, |raw| json::from_str(raw.get()))?;

        Ok(Self { inner })
    }

    /// Access this handle's data
    pub fn data(&self) -> &Created {
        self.inner.borrow_handle()
    }

    /// Consume the handle returning the backing
    /// storage
    pub fn into_inner(self) -> Box<RawJson> {
        self.inner.into_heads().store
    }
}

impl TryFrom<Box<RawJson>> for CreatedHandle {
    type Error = JsonError;

    fn try_from(value: Box<RawJson>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

// Delegate the serializer to the internal handle
impl Serialize for CreatedHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data().serialize(serializer)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Created<'a> {
    #[serde(with = "common::id")]
    id: u64,
    key: &'a str,
    #[serde(rename = "self")]
    self_link: &'a str,
    // Only exists if a transition was requested in the associated request
    #[serde(skip_serializing_if = "Option::is_none")]
    transition: Option<NestedResponse<'a>>,
}

mod handle {
    use super::*;
    use ouroboros::self_referencing as ouroboros;

    #[ouroboros(pub_extras)]
    #[derive(Debug)]
    pub(super) struct CreatedInner {
        store: Box<RawJson>,
        #[borrows(store)]
        pub(super) handle: Created<'this>,
    }
}

#[cfg(test)]
pub(crate) mod types {
    use serde_json::{json, Value as Json};

    pub fn created() -> Json {
        json!({
            "id": "42",
            "self": "foo",
            "key": "foo",
            "transition": null,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as Json;

    #[test]
    fn deserialize_created_handle() {
        let json = jbytes(types::created());

        let handle: Result<CreatedHandle, _> = deserialize(&json);

        assert!(handle.is_ok())
    }

    #[test]
    fn deserialize_created() {
        let json = jbytes(types::created());

        let created: Result<Created, _> = deserialize(&json);

        assert!(created.is_ok())
    }

    fn jbytes(json: Json) -> Vec<u8> {
        serde_json::to_vec(&json)
            .expect("Failed to serialize in models/issue/create tests... this is a bug")
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
