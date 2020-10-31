use super::*;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Get {
    #[serde(skip_serializing_if = "none_or_empty")]
    fields: Option<CommaDelimited>,
    #[serde(skip_serializing_if = "none_or_empty")]
    expand: Option<CommaDelimited>,
    #[serde(rename = "fieldsByKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    fields_by_key: Option<bool>,
    #[serde(skip_serializing_if = "none_or_empty")]
    properties: Option<CommaDelimited>,
    #[serde(rename = "updateHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    update_history: Option<bool>,
}

impl Get {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fields<I, T>(&mut self, fields: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        Self::append_delimited(
            &mut self.fields,
            fields.into_iter().map(|s| Element::from(s.as_ref())),
        );
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

    pub fn fields_by_key(&mut self, by_key: impl Into<Option<bool>>) -> &mut Self {
        self.fields_by_key = by_key.into().filter(|b| *b);
        self
    }

    pub fn properties<T, I>(&mut self, properties: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<str>,
    {
        Self::append_delimited(
            &mut self.properties,
            properties.into_iter().map(|s| Element::from(s.as_ref())),
        );
        self
    }

    pub fn update_history(&mut self, update: impl Into<Option<bool>>) -> &mut Self {
        self.update_history = update.into().filter(|u| *u);
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
        let get = Get::new();
        let req = generate(&get);

        assert_eq!(req.url().query(), None);
    }

    #[test]
    fn single() {
        let get = Get::new().with(|this| this.fields(Some("value")));
        let req = generate(&get);
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(query, "fields=value")
    }

    #[test]
    fn multiple() {
        let get = Get::new().with(|this| this.expand(Some("value")).update_history(true));
        let req = generate(&get);
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(query, "expand=value&updateHistory=true")
    }

    #[test]
    fn complex() {
        let get = Get::new().with(|this| {
            this.properties(&["foo", "bar", "baz"])
                .fields_by_key(true)
                .fields(Some("field"))
                .update_history(false)
        });
        let req = generate(&get);
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(
            query,
            "fields=field&fieldsByKeys=true&properties=foo%2Cbar%2Cbaz"
        )
    }

    /// Added after a regression whereby "empty" iterators
    /// could add an empty struct as a Some() variant causing
    /// invalid query serialization
    #[test]
    fn empty_values() {
        const EMPTY: Option<String> = None;

        let req = generate(&*Get::new().fields(EMPTY));
        assert_eq!(req.url().query(), None);

        let req = generate(&*Get::new().expand(EMPTY));
        assert_eq!(req.url().query(), None);

        let req = generate(&*Get::new().properties(EMPTY));
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
