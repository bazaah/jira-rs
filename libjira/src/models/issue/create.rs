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

