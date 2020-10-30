use super::*;

/// Options for querying the create variant of Jira's issue metadata
/// endpoint.
#[derive(Debug, Default, Clone, Serialize)]
pub struct MetaCreate {
    #[serde(rename = "projectIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    project_ids: Option<CommaDelimited>,
    #[serde(rename = "projectKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    project_keys: Option<CommaDelimited>,
    #[serde(rename = "issuetypeIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    issuetype_ids: Option<CommaDelimited>,
    #[serde(rename = "issuetypeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    issuetype_keys: Option<CommaDelimited>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<CommaDelimited>,
}

impl MetaCreate {
    /// Instantiate a new, empty options set
    pub fn new() -> Self {
        Self::default()
    }

    /// Narrow the returned issue metadata to issues belonging to the
    /// given projects, by key. These typically correspond to the
    /// human readable names of projects.
    ///
    /// Note that entries are additive, i.e given projects `P1` and
    /// `P2`, the endpoint will return _issues ∈ P1 ∪ P2_. Additionally,
    /// entries are additive with `project_ids` entries.
    pub fn project_keys<I, T>(&mut self, keys: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        Self::append_delimited(
            &mut self.project_keys,
            keys.into_iter().map(|s| Element::from(s.as_ref())),
        );
        self
    }

    /// Narrow the returned issue metadata to issues belonging to the
    /// given projects, by id. These are the programmatic representations
    /// of Jira projects.
    ///
    /// Note that entries are additive, i.e given projects `P1` and
    /// `P2`, the endpoint will return _issues ∈ P1 ∪ P2_. Additionally,
    /// entries are additive with `project_keys` entries.
    pub fn project_ids<I, T>(&mut self, ids: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<u64>,
    {
        Self::append_delimited(&mut self.project_ids, ids.into_iter().map(Into::into));
        self
    }

    /// Narrow the returned issue metadata to issues of the given type keys.
    /// Typical examples include `Theme`(s), `Spike`(s) or `Story`(ies). These
    /// entries are case sensitive and tend to vary between Jira instances.
    /// If possible, address issue types using the `_id` variant.
    ///
    /// Note that entries are additive with entries of `issuetype_ids`:
    /// _types ∈ KEYS ∪ IDS_
    pub fn issuetype_keys<I, T>(&mut self, keys: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        Self::append_delimited(
            &mut self.issuetype_keys,
            keys.into_iter().map(|s| Element::from(s.as_ref())),
        );
        self
    }

    /// Narrow the returned issue metadata to issues of the given type ids.
    ///
    /// Note that entries are additive with entries of `issuetype_keys`:
    /// _types ∈ KEYS ∪ IDS_
    pub fn issuetype_ids<I, T>(&mut self, ids: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<u64>,
    {
        Self::append_delimited(&mut self.issuetype_ids, ids.into_iter().map(Into::into));
        self
    }

    /// The Jira expandable for this endpoint. This one recognizes
    /// `projects.issuetypes.fields`. You can include any subsegment, however
    /// the effects are only noticeable with all three.
    ///
    /// Not including this in a metadata query will return a very limited
    /// output -- which can be good for exploring possible issue candidates
    /// relatively cheaply, before expanding the spec of your target issues.
    pub fn expand<I, T>(&mut self, expand: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        Self::append_delimited(
            &mut self.expand,
            expand.into_iter().map(|s| Element::from(s.as_ref())),
        );
        self
    }

    /// Helper function for emulating a builder pattern
    pub fn with<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Self) -> &mut Self,
    {
        let mut this = self;
        f(&mut this);
        this
    }

    fn append_delimited<I, T>(f: &mut Option<CommaDelimited>, iter: I)
    where
        I: Iterator<Item = T>,
        T: Into<Element>,
    {
        match f {
            Some(ref mut item) => item.extend(iter.map(Into::into)),
            None => {
                *f = iter.map(Into::into).fold(None, |mut o, elem| {
                    o.get_or_insert_with(|| CommaDelimited::new()).append(elem);
                    o
                })
            }
        }
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn empty() {
        let get = MetaCreate::new();
        let req = generate(&get);

        assert_eq!(req.url().query(), None);
    }

    #[test]
    fn single() {
        let get = MetaCreate::new().with(|this| this.expand(Some("value")));
        let req = generate(&get);
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(query, "expand=value")
    }

    #[test]
    fn multiple() {
        let get = MetaCreate::new()
            .with(|this| this.issuetype_ids(Some(42u64)).project_keys(Some("foo")));
        let req = generate(&get);
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(query, "projectKeys=foo&issuetypeIds=42")
    }

    #[test]
    fn complex() {
        let get = MetaCreate::new().with(|this| {
            this.project_keys(&["key1", "key2"])
                .issuetype_ids((&[0u32, 10, 30]).iter().copied())
                .expand(Some("value"))
        });
        let req = generate(&get);
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(
            query,
            "projectKeys=key1%2Ckey2&issuetypeIds=0%2C10%2C30&expand=value"
        )
    }

    /// Added after a regression whereby "empty" iterators
    /// could add an empty struct as a Some() variant causing
    /// invalid query serialization
    #[test]
    fn empty_values() {
        const EMPTY_S: Option<String> = None;
        const EMPTY_N: Option<u64> = None;

        let req = generate(&*MetaCreate::new().project_ids(EMPTY_N));
        assert_eq!(req.url().query(), None);

        let req = generate(&*MetaCreate::new().project_keys(EMPTY_S));
        assert_eq!(req.url().query(), None);

        let req = generate(&*MetaCreate::new().issuetype_ids(EMPTY_N));
        assert_eq!(req.url().query(), None);

        let req = generate(&*MetaCreate::new().issuetype_keys(EMPTY_S));
        assert_eq!(req.url().query(), None);

        let req = generate(&*MetaCreate::new().expand(EMPTY_S));
        assert_eq!(req.url().query(), None);
    }

    fn generate(s: impl Serialize) -> reqwest::Request {
        reqwest::Client::new()
            .get("http://localhost")
            .query(&s)
            .build()
            .expect("a valid request")
    }
}
