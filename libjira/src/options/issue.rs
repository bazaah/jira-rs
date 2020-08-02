use super::*;

pub use super::ValidateQuery;

#[derive(Debug, Default, Clone)]
pub struct Get<'a> {
    with_fields: Option<Vec<SmolCow<'a, str>>>,
    expand: Option<Vec<SmolCow<'a, str>>>,
    fields_by_key: Option<bool>,
    properties: Option<Vec<SmolCow<'a, str>>>,
    update_history: Option<bool>,
}

#[derive(Debug, Default, Clone)]
pub struct Search<'a> {
    jql: Option<SmolCow<'a, str>>,
    start_at: Option<u32>,
    max_results: Option<u32>,
    validate: Option<ValidateQuery>,
    with_fields: Option<Vec<SmolCow<'a, str>>>,
    expand: Option<Vec<SmolCow<'a, str>>>,
    properties: Option<Vec<SmolCow<'a, str>>>,
    fields_by_key: Option<bool>,
}

impl Get<'static> {
    pub fn into_owned<'a>(get: Get<'a>) -> Self {
        Get {
            with_fields: get
                .with_fields
                .map(|v| v.into_iter().map(|s| SmolCow::Owned(s.to_smol())).collect()),
            expand: get
                .expand
                .map(|v| v.into_iter().map(|s| SmolCow::Owned(s.to_smol())).collect()),
            fields_by_key: get.fields_by_key,
            properties: get
                .properties
                .map(|v| v.into_iter().map(|s| SmolCow::Owned(s.to_smol())).collect()),
            update_history: get.update_history,
        }
    }
}

impl<'a> Get<'a> {
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

    pub fn with_fields<T: 'a>(self, fields: &'a [T]) -> Self
    where
        T: AsRef<str>,
    {
        let mut this = self;
        this.with_fields = Some(fields.iter().map(|s| s.as_ref().into()).collect());
        this
    }

    pub fn expand<T: 'a>(self, expand: &'a [T]) -> Self
    where
        T: AsRef<str>,
    {
        let mut this = self;
        this.expand = Some(expand.iter().map(|s| s.as_ref().into()).collect());
        this
    }

    pub fn properties<T: 'a>(self, properties: &'a [T]) -> Self
    where
        T: AsRef<str>,
    {
        let mut this = self;
        this.properties = Some(properties.iter().map(|s| s.as_ref().into()).collect());
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

impl Search<'static> {
    pub fn into_owned<'a>(search: Search<'a>) -> Self {
        Search {
            jql: search.jql.map(|s| SmolCow::Owned(s.to_smol())),
            start_at: search.start_at,
            max_results: search.max_results,
            validate: search.validate,
            with_fields: search
                .with_fields
                .map(|v| v.into_iter().map(|s| SmolCow::Owned(s.to_smol())).collect()),
            expand: search
                .expand
                .map(|v| v.into_iter().map(|s| SmolCow::Owned(s.to_smol())).collect()),
            fields_by_key: search.fields_by_key,
            properties: search
                .properties
                .map(|v| v.into_iter().map(|s| SmolCow::Owned(s.to_smol())).collect()),
        }
    }
}

impl<'a> Search<'a> {
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

    pub fn jql<T: 'a>(self, jql: T) -> Self
    where
        T: Into<SmolCow<'a, str>>,
    {
        let mut this = self;
        this.jql = Some(jql.into());
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

    pub fn with_fields<T: 'a>(self, fields: &'a [T]) -> Self
    where
        T: AsRef<str>,
    {
        let mut this = self;
        this.with_fields = Some(fields.iter().map(|s| s.as_ref().into()).collect());
        this
    }

    pub fn expand<T: 'a>(self, expand: &'a [T]) -> Self
    where
        T: AsRef<str>,
    {
        let mut this = self;
        this.expand = Some(expand.iter().map(|s| s.as_ref().into()).collect());
        this
    }

    pub fn properties<T: 'a>(self, properties: &'a [T]) -> Self
    where
        T: AsRef<str>,
    {
        let mut this = self;
        this.properties = Some(properties.iter().map(|s| s.as_ref().into()).collect());
        this
    }

    pub fn fields_by_key(self, by_key: bool) -> Self {
        let mut this = self;
        this.fields_by_key = Some(by_key);
        this
    }
}

impl<'a> ApiOptions for Get<'a> {
    fn valid_options(&self) -> &[OptRef] {
        Self::VALID.as_ref()
    }

    fn with_fields(&self) -> Option<&[SmolCow<str>]> {
        self.with_fields.as_deref()
    }

    fn expand(&self) -> Option<&[SmolCow<str>]> {
        self.expand.as_deref()
    }

    fn properties(&self) -> Option<&[SmolCow<str>]> {
        self.properties.as_deref()
    }

    fn fields_by_key(&self) -> Option<bool> {
        self.fields_by_key
    }

    fn update_history(&self) -> Option<bool> {
        self.update_history
    }
}

impl<'a> ApiOptions for Search<'a> {
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

    fn with_fields(&self) -> Option<&[SmolCow<str>]> {
        self.with_fields.as_deref()
    }

    fn expand(&self) -> Option<&[SmolCow<str>]> {
        self.expand.as_deref()
    }

    fn properties(&self) -> Option<&[SmolCow<str>]> {
        self.properties.as_deref()
    }

    fn fields_by_key(&self) -> Option<bool> {
        self.fields_by_key
    }
}
