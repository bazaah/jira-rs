use std::{borrow::Cow, collections::HashMap, fmt, ops::Deref};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};

/// Attempt to deserialize a Cow as borrowed where possible. This function can be combined with the
/// serde attributes 'borrow' and 'deserialize_with' to avoid allocating on every deserialization
pub fn deserialize<'de, D>(deserializer: D) -> Result<Cow<'de, str>, D::Error>
where
    D: Deserializer<'de>,
{
    let specialized = CowStr::deserialize(deserializer)?;

    Ok(specialized.0)
}

/// Attempt to deserialize an optional Cow.
///
/// Note, due to a quirk in serde's derive magic you *must* add a 'default' attribute in addition to
/// the 'borrow' and 'deserialize_with' ones to keep the field truly optional.
///
/// E.g: serde(default, borrow, deserialize_with = "cow::deserialize_option", ...others)
pub fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<Cow<'de, str>>, D::Error>
where
    D: Deserializer<'de>,
{
    let specialized: Option<CowStr> = Deserialize::deserialize(deserializer)?;

    Ok(specialized.map(|cs| cs.0))
}

/// Deserialize a hash map with both keys and values being Cow
pub fn deserialize_kv<'de, D>(
    deserializer: D,
) -> Result<HashMap<Cow<'de, str>, Cow<'de, str>>, D::Error>
where
    D: Deserializer<'de>,
{
    let specialized: HashMap<CowStr, CowStr> = HashMap::deserialize(deserializer)?;

    let o = specialized.into_iter().map(|(k, v)| (k.0, v.0)).collect();

    Ok(o)
}

/// Deserialize a hash map with the k being Cow and the value implementing Deserialize
pub fn deserialize_k<'de, D, T>(deserializer: D) -> Result<HashMap<Cow<'de, str>, T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let specialized: HashMap<CowStr, T> = HashMap::deserialize(deserializer)?;

    let o = specialized.into_iter().map(|(k, v)| (k.0, v)).collect();

    Ok(o)
}

/// A specialized version of Cow specifically for borrowing &strs when deserializing, by default
/// Serde will never borrow for Cow and thus we must new-type our way around this annoyance
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct CowStr<'a>(pub Cow<'a, str>);

impl<'a> CowStr<'a> {
    pub const fn from_str(s: &'a str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

impl<'a> Into<Cow<'a, str>> for CowStr<'a> {
    fn into(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a> Deref for CowStr<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a> PartialEq<str> for CowStr<'a> {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl<'a, 'b> PartialEq<&'b str> for CowStr<'a> {
    fn eq(&self, other: &&'b str) -> bool {
        self.0 == *other
    }
}

impl<'de> Deserialize<'de> for CowStr<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CowStrVisitor)
    }
}

struct CowStrVisitor;

impl<'de> Visitor<'de> for CowStrVisitor {
    type Value = CowStr<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CowStr(Cow::Borrowed(v)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CowStr(Cow::Owned(v.to_owned())))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CowStr(Cow::Owned(v)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct ShouldBorrow<'a> {
        #[serde(borrow, deserialize_with = "deserialize")]
        b: Cow<'a, str>,
    }

    const INPUT: &'static str = r#"{"b": "I should be borrowed"}"#;

    #[test]
    fn deserialize_is_borrowed() {
        let test: ShouldBorrow = serde_json::from_str(INPUT).unwrap();

        match test.b {
            Cow::Borrowed(_) => {}
            Cow::Owned(_) => panic!("CowStr failed to deserialize a borrowed str"),
        }
    }
}
