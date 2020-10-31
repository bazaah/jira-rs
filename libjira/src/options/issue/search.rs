use super::*;

/// Options for searching Jira issues using [JQL](https://support.atlassian.com/jira-software-cloud/docs/use-advanced-search-with-jira-query-language-jql)
/// queries.
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
    #[serde(skip_serializing_if = "none_or_empty")]
    fields: Option<CommaDelimited>,
    #[serde(skip_serializing_if = "none_or_empty")]
    expand: Option<CommaDelimited>,
    #[serde(skip_serializing_if = "none_or_empty")]
    properties: Option<CommaDelimited>,
    #[serde(rename = "fieldsByKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    fields_by_key: Option<bool>,
}

impl Search {
    /// Instantiate a new, empty options set
    pub fn new() -> Self {
        Self::default()
    }

    /// The JQL query to send to the endpoint. Note that not setting this
    /// field will **automatically fail** the request.
    pub fn jql<T>(&mut self, jql: impl Into<Option<T>>) -> &mut Self
    where
        T: ToString,
    {
        self.jql = jql.into().map(|s| s.to_string());
        self
    }

    /// Only send results starting from the given number.
    ///
    /// For example, assuming a query normally returns `0..=N` matching issues,
    /// setting this to `S` will instead return `S..=N` results.
    pub fn start_at(&mut self, start_at: impl Into<Option<u32>>) -> &mut Self {
        self.start_at = start_at.into();
        self
    }

    /// Sets the limit on number of results returned in a single quest for a given
    /// query. By default Jira instances set this to 50, but this is configurable by
    /// site administrators. Combining this setting and `start_at` allows you to paginate
    /// results.
    pub fn max_results(&mut self, max_results: impl Into<Option<u32>>) -> &mut Self {
        self.max_results = max_results.into().filter(|u| *u != 0);
        self
    }

    /// Set the validation level for this query's JQL. See `ValidateQuery` for more
    /// information on available levels.
    pub fn validate(&mut self, validate: impl Into<Option<ValidateQuery>>) -> &mut Self {
        self.validate = validate.into();
        self
    }

    /// Narrows the returned fields of issues returned by the given
    /// JQL query. By default, Jira will return _all_ fields accessible
    /// by the request's permissions, which can number in the hundreds,
    /// significantly bloating responses.
    ///
    /// If you know what you are looking for, you can greatly improve
    /// roundtrip performance by leveraging this setting.
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

    /// The Jira expandable for this endpoint. This one can take serveral
    /// defined expands, and may additionally take others depending on
    /// what search plugins the Jira instance has.
    ///
    /// For more information on the defined expands see the constants
    /// in `self::expands`.
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

    /// Returned fields will be referenced by their key instead of their id.
    pub fn fields_by_key(&mut self, by_key: impl Into<Option<bool>>) -> &mut Self {
        self.fields_by_key = by_key.into().filter(|b| *b);
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

pub mod expands {
    /// Returns field values rendered in HTML format.
    pub const RENDERED_FIELDS: &str = "renderedFields";

    /// Returns the display name of each field.
    pub const NAMES: &str = "names";

    /// Returns the schema describing a field type.
    pub const SCHEMA: &str = "schema";

    /// Returns all possible transitions for the issue.
    pub const TRANSITIONS: &str = "transitions";

    /// Returns all possible operations for the issue.
    pub const OPERATIONS: &str = "operations";

    /// Returns information about how each field can be edited.
    pub const EDITMETA: &str = "editmeta";

    /// Returns a list of recent updates to an issue, sorted by date, starting from the most recent.
    pub const CHANGELOG: &str = "changelog";

    /// Instead of `fields`, returns `versionedRepresentations`, a JSON array containing each
    /// version of a field's value, with the highest numbered item representing the most recent version.
    pub const VERSIONED_REPR: &str = "versionedRepresentations";
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
