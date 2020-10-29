use {
    serde::{
        de::{Deserializer, Error, IgnoredAny, MapAccess, Visitor},
        Deserialize,
    },
    std::fmt,
};

/// Representation of an empty response from a JIRA server
/// Contains no data and can be converted to an empty Rust tuple '()'
pub struct Empty;

impl<'de> Deserialize<'de> for Empty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(EmptyVisitor)
    }
}

struct EmptyVisitor;

impl<'de> Visitor<'de> for EmptyVisitor {
    type Value = Empty;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("An empty JSON map: '{}'")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        if let Some((_k, _v)) = map.next_entry::<IgnoredAny, IgnoredAny>()? {
            return Err(Error::invalid_length(
                map.size_hint().unwrap_or(1),
                &"Expected an empty map",
            ));
        }

        Ok(Empty)
    }
}

impl From<Empty> for () {
    fn from(_: Empty) -> Self {}
}
