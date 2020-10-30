use super::*;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Search {
    #[serde(skip_serializing_if = "Option::is_none")]
    jql: Option<String>,
    #[serde(rename = "startAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    start_at: Option<u32>,
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_results: Option<u32>,
    #[serde(rename = "validateQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    validate: Option<ValidateQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<CommaDelimited>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<CommaDelimited>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<CommaDelimited>,
    #[serde(rename = "fieldsByKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    fields_by_key: Option<bool>,
}

impl Search {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn jql<T>(&mut self, jql: impl Into<Option<T>>) -> &mut Self
    where
        T: ToString,
    {
        self.jql = jql.into().map(|s| s.to_string());
        self
    }

    pub fn start_at(&mut self, start_at: impl Into<Option<u32>>) -> &mut Self {
        self.start_at = start_at.into();
        self
    }

    pub fn max_results(&mut self, max_results: impl Into<Option<u32>>) -> &mut Self {
        self.max_results = max_results.into().filter(|u| *u != 0);
        self
    }

    pub fn validate(&mut self, validate: impl Into<Option<ValidateQuery>>) -> &mut Self {
        self.validate = validate.into();
        self
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

    pub fn properties<I, T>(&mut self, properties: I) -> &mut Self
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

    pub fn fields_by_key(&mut self, by_key: impl Into<Option<bool>>) -> &mut Self {
        self.fields_by_key = by_key.into().filter(|b| *b);
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
        let search = Search::new();
        let req = generate(&search);

        assert_eq!(req.url().query(), None);
    }

    #[test]
    fn single() {
        let search = Search::new().with(|this| this.expand(Some("value")));
        let req = generate(&search);
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(query, "expand=value")
    }

    #[test]
    fn multiple() {
        let search = Search::new().with(|this| this.max_results(100u32).fields_by_key(true));
        let req = generate(&search);
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(query, "maxResults=100&fieldsByKeys=true")
    }

    #[test]
    fn complex() {
        let search = Search::new().with(|this| {
            this.start_at(80)
                .validate(ValidateQuery::Strict)
                .properties(&["foo", "bar"])
        });
        let req = generate(&search);
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(
            query,
            "startAt=80&validateQuery=strict&properties=foo%2Cbar"
        )
    }

    /// Added after a regression whereby "empty" iterators
    /// could add an empty struct as a Some() variant causing
    /// invalid query serialization
    #[test]
    fn empty_values() {
        const EMPTY: Option<String> = None;

        let req = generate(&*Search::new().expand(EMPTY));
        assert_eq!(req.url().query(), None);

        let req = generate(&*Search::new().fields(EMPTY));
        assert_eq!(req.url().query(), None);

        let req = generate(&*Search::new().properties(EMPTY));
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
