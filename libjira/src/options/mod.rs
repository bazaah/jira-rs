use {
    issue::ValidateQuery,
    reqwest::RequestBuilder,
    serde::{Serialize, Serializer},
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

/// Glue trait between Reqwest's .query() serializer
/// and this library's various *Options structs
///
/// This serializer (urlencoded) is very particular about what types
/// it will serialize. Namely, it only accepts a slice containing <N>
/// two-element tuples where each element serializes as a str of
/// some kind.
pub(crate) trait ToQuery<'a> {
    type Queries: Iterator<Item = (&'a str, OptionSerialize<'a>)>;

    fn to_queries(&'a self) -> Self::Queries;

    fn append_request(&'a self, request: RequestBuilder) -> RequestBuilder {
        self.to_queries()
            .fold(request, |req, query| req.query(&[query]))
    }
}

/// Wrapper around the various types used by *Options structs
///
/// Each variant must serialize as something Reqwest's query
/// serializer accepts
#[derive(Debug, Serialize, Copy, Clone)]
#[serde(untagged)]
pub(crate) enum OptionSerialize<'a> {
    CommaDelimited(&'a str),
    Unsigned(u64),
    Bool(bool),
    Text(&'a str),
    Validate(ValidateQuery),
}

impl From<ValidateQuery> for OptionSerialize<'_> {
    fn from(v: ValidateQuery) -> Self {
        Self::Validate(v)
    }
}

impl<'a> From<&'a CommaDelimited> for OptionSerialize<'a> {
    fn from(v: &'a CommaDelimited) -> Self {
        Self::CommaDelimited(v.as_ref())
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

#[derive(Debug, Default, Clone)]
struct CommaDelimited {
    buffer: String,
}

impl CommaDelimited {
    fn new() -> Self {
        Self::default()
    }

    fn add_item<T>(&mut self, item: T) -> &mut Self
    where
        T: DelimitedItem,
    {
        match self.buffer.is_empty() {
            // Writing to String never returns an error
            true => item.write_item(&mut self.buffer),
            false => {
                self.buffer.push_str(",");
                item.write_item(&mut self.buffer);
            }
        }

        self
    }

    fn from_iter<I, T>(iter: I) -> Self
    where
        I: Iterator<Item = T> + Clone,
        T: DelimitedItem,
    {
        let mut this = Self::new();
        // Attempt to limit potential allocations to 1
        let len = iter.clone().fold(0, |acc, item| acc + item.length() + 1);

        this.buffer.reserve(len);

        iter.for_each(|item| {
            this.add_item(item);
        });

        this
    }

    fn from_slice<T>(items: &[T]) -> Self
    where
        T: AsRef<str>,
    {
        Self::from_iter(items.iter().map(|i| i.as_ref()))
    }
}

impl AsRef<str> for CommaDelimited {
    fn as_ref(&self) -> &str {
        self.buffer.as_ref()
    }
}

impl Serialize for CommaDelimited {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

trait DelimitedItem {
    fn length(&self) -> usize;

    fn write_item<W>(&self, writer: &mut W)
    where
        W: std::fmt::Write;
}

impl DelimitedItem for &str {
    fn length(&self) -> usize {
        self.len()
    }

    fn write_item<W>(&self, writer: &mut W)
    where
        W: std::fmt::Write,
    {
        writer.write_str(self).unwrap()
    }
}

impl DelimitedItem for u64 {
    fn length(&self) -> usize {
        itoa::Buffer::new().format(*self).len()
    }

    fn write_item<W>(&self, writer: &mut W)
    where
        W: std::fmt::Write,
    {
        itoa::fmt(writer, *self).unwrap()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    const ALL: &'static [(&'static str, OptionSerialize<'static>)] = &[
        ("key", OptionSerialize::Text("value")),
        ("bool", OptionSerialize::Bool(true)),
        ("comma", OptionSerialize::CommaDelimited("comma,delimited")),
        ("validate", OptionSerialize::Validate(ValidateQuery::Strict)),
        ("uint", OptionSerialize::Unsigned(42)),
    ];
    const EMPTY: &'static [(&'static str, OptionSerialize<'static>)] = &[];

    #[test]
    fn to_query_none() {
        let r = request(EMPTY);
        let queries = r.url().query();

        dbg!(queries);
        assert!(queries.is_none())
    }

    #[test]
    fn to_query_all() {
        let r = request(ALL);
        let queries = r.url().query().expect("A non empty query string");

        dbg!(queries);
        assert_eq!(
            queries,
            "key=value&bool=true&comma=comma%2Cdelimited&validate=strict&uint=42"
        )
    }

    #[derive(Debug, Clone)]
    struct TestIter<'a> {
        iter: std::slice::Iter<'a, (&'a str, OptionSerialize<'a>)>,
    }

    impl<'a> TestIter<'a> {
        fn new(iter: std::slice::Iter<'a, (&'a str, OptionSerialize<'a>)>) -> Self {
            Self { iter }
        }
    }

    impl<'a> Iterator for TestIter<'a> {
        type Item = (&'a str, OptionSerialize<'a>);

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.next().map(|i| i.clone())
        }
    }

    impl<'a> ToQuery<'a> for TestIter<'a> {
        type Queries = Self;

        fn to_queries(&'a self) -> Self::Queries {
            self.clone()
        }
    }

    fn request(from: &[(&str, OptionSerialize)]) -> reqwest::Request {
        TestIter::new(from.iter())
            .append_request(generate())
            .build()
            .expect("valid request")
    }

    fn generate() -> RequestBuilder {
        reqwest::Client::new().get("http://localhost")
    }
}
