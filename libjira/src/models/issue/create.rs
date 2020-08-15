use {
    super::*,
    json::{value::RawValue as RawJson, Error as JsonError},
    serde::Serializer,
    serde_json as json,
};

#[derive(Debug, Deserialize)]
#[serde(try_from = "Box<RawJson>")]
pub struct Created {
    // This handle must never be exposed in the public API
    inner: handle::CreatedHandle,
}

impl Created {
    /// Try instantiate a new handle with the given backing JSON
    pub fn try_new(store: Box<RawJson>) -> Result<Self, JsonError> {
        let inner =
            handle::CreatedHandle::try_new_or_drop(store, |json| json::from_str(json.get()))?;

        Ok(Self { inner })
    }

    /// Access this handle's data
    pub fn data(&self) -> &CreatedRef {
        self.inner.suffix()
    }

    /// Consume the handle returning the backing
    /// storage
    pub fn into_inner(self) -> Box<RawJson> {
        self.inner.into_head()
    }
}

impl TryFrom<Box<RawJson>> for Created {
    type Error = JsonError;

    fn try_from(value: Box<RawJson>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

// Delegate the serializer to the internal handle
impl Serialize for Created {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data().serialize(serializer)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreatedRef<'a> {
    id: u64,
    key: &'a str,
    self_link: &'a str,
    transition: NestedResponse<'a>,
}

rental! {
    mod handle {
        use super::*;

        #[rental(debug, covariant)]
        pub(super) struct CreatedHandle {
            store: Box<RawJson>,
            handle: CreatedRef<'store>
        }
    }
}
