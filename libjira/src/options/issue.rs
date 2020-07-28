use super::*;

#[derive(Debug, Default, Clone)]
pub struct IssueOptions {
    jql: Option<String>,
    start_at: Option<u32>,
    max_results: Option<u32>,
    validate: Option<ValidateQuery>,
    with_fields: Option<Vec<String>>,
    expand: Option<Vec<String>>,
    properties: Option<Vec<String>>,
    fields_by_key: Option<bool>,
}

impl IssueOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn jql<T>(self, jql: T) -> Self
    where
        T: ToString,
    {
        let mut this = self;
        this.jql = Some(jql.to_string());
        this
    }

    pub fn start_at(self, start_at: u32) -> Self {
        let mut this = self;
        this.start_at = Some(start_at);
        this
    }

    pub fn max_results(self, max_results: u32) -> Self {
        let mut this = self;
        this.max_results = Some(max_results);
        this
    }

    pub fn validate(self, validate: ValidateQuery) -> Self {
        let mut this = self;
        this.validate = Some(validate);
        this
    }

    pub fn with_fields(self, fields: Vec<String>) -> Self {
        let mut this = self;
        this.with_fields = Some(fields);
        this
    }

    pub fn expand(self, expand: Vec<String>) -> Self {
        let mut this = self;
        this.expand = Some(expand);
        this
    }

    pub fn properties(self, properties: Vec<String>) -> Self {
        let mut this = self;
        this.properties = Some(properties);
        this
    }

    pub fn fields_by_key(self, by_key: bool) -> Self {
        let mut this = self;
        this.fields_by_key = Some(by_key);
        this
    }
}

static VALID_OPTIONS: [OptRef; 8] = [
    OptRef::Jql,
    OptRef::StartAt,
    OptRef::MaxResults,
    OptRef::ValidateQuery,
    OptRef::WithFields,
    OptRef::Expand,
    OptRef::Properties,
    OptRef::FieldsByKey,
];

impl ApiOptions for IssueOptions {
    fn valid_options(&self) -> &[OptRef] {
        VALID_OPTIONS.as_ref()
    }

    fn jql(&self) -> Option<&str> {
        self.jql.as_deref()
    }

    fn start_at(&self) -> Option<u32> {
        self.start_at
    }

    fn max_results(&self) -> Option<u32> {
        self.max_results
    }

    fn validate_query(&self) -> Option<ValidateQuery> {
        self.validate
    }

    fn with_fields(&self) -> Option<&[String]> {
        self.with_fields.as_deref()
    }

    fn expand(&self) -> Option<&[String]> {
        self.expand.as_deref()
    }

    fn properties(&self) -> Option<&[String]> {
        self.properties.as_deref()
    }

    fn fields_by_key(&self) -> Option<bool> {
        self.fields_by_key
    }
}