use super::*;

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
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn project_ids<I, T>(&mut self, ids: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<u64>,
    {
        Self::append_delimited(&mut self.project_ids, ids.into_iter().map(Into::into));
        self
    }

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

    pub fn issuetype_ids<I, T>(&mut self, ids: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<u64>,
    {
        Self::append_delimited(&mut self.issuetype_ids, ids.into_iter().map(Into::into));
        self
    }

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
            None => *f = Some(CommaDelimited::from_iter(iter.map(Into::into))),
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

    fn generate(s: impl Serialize) -> reqwest::Request {
        reqwest::Client::new()
            .get("http://localhost")
            .query(&s)
            .build()
            .expect("a valid request")
    }
}
