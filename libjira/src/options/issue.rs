use super::*;

pub use super::ValidateQuery;

#[derive(Debug, Default, Clone)]
pub struct Get {
    pub with_fields: Option<Vec<String>>,
    pub expand: Option<Vec<String>>,
    pub fields_by_key: Option<bool>,
    pub properties: Option<Vec<String>>,
    pub update_history: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct Search {
    pub jql: Option<String>,
    pub start_at: Option<u32>,
    pub max_results: Option<u32>,
    pub validate: Option<ValidateQuery>,
    pub with_fields: Option<Vec<String>>,
    pub expand: Option<Vec<String>>,
    pub properties: Option<Vec<String>>,
    pub fields_by_key: Option<bool>,
}

impl Get {
    const VALID: [OptRef; 5] = [
        OptRef::WithFields,
        OptRef::Expand,
        OptRef::FieldsByKey,
        OptRef::Properties,
        OptRef::UpdateHistory,
    ];

    pub fn new() -> Self {
        Self::default()
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

    pub fn update_history(self, update: bool) -> Self {
        let mut this = self;
        this.update_history = Some(update);
        this
    }
}

impl Search {
    const VALID: [OptRef; 8] = [
        OptRef::Jql,
        OptRef::StartAt,
        OptRef::MaxResults,
        OptRef::ValidateQuery,
        OptRef::WithFields,
        OptRef::Expand,
        OptRef::Properties,
        OptRef::FieldsByKey,
    ];

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

impl ApiOptions for Get {
    fn valid_options(&self) -> &[OptRef] {
        Self::VALID.as_ref()
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

    fn update_history(&self) -> Option<bool> {
        self.update_history
    }
}

impl ApiOptions for Search {
    fn valid_options(&self) -> &[OptRef] {
        Self::VALID.as_ref()
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
