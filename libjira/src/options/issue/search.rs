use super::*;

#[derive(Debug, Default, Clone)]
pub struct Search {
    jql: Option<String>,
    start_at: Option<u32>,
    max_results: Option<u32>,
    validate: Option<ValidateQuery>,
    with_fields: Option<CommaDelimited>,
    expand: Option<CommaDelimited>,
    properties: Option<CommaDelimited>,
    fields_by_key: Option<bool>,
}

impl Search {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn jql<T>(self, jql: Option<T>) -> Self
    where
        T: AsRef<str>,
    {
        let mut this = self;
        this.jql = jql.map(|s| s.as_ref().into());
        this
    }

    pub fn start_at(self, start_at: Option<u32>) -> Self {
        let mut this = self;
        this.start_at = start_at;
        this
    }

    pub fn max_results(self, max_results: Option<u32>) -> Self {
        let mut this = self;
        this.max_results = max_results;
        this
    }

    pub fn validate(self, validate: Option<ValidateQuery>) -> Self {
        let mut this = self;
        this.validate = validate;
        this
    }

    pub fn with_fields<'a, I>(self, fields: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        let mut this = self;
        this.with_fields = fields.map(|i| CommaDelimited::from_iter(i));
        this
    }

    pub fn expand<'a, I>(self, expand: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        let mut this = self;
        this.expand = expand.map(|i| CommaDelimited::from_iter(i));
        this
    }

    pub fn properties<'a, I>(self, properties: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        let mut this = self;
        this.properties = properties.map(|i| CommaDelimited::from_iter(i));
        this
    }

    pub fn fields_by_key(self, by_key: Option<bool>) -> Self {
        let mut this = self;
        this.fields_by_key = by_key.filter(|b| *b);
        this
    }
}

impl<'a> ToQuery<'a> for Search {
    type Queries = SearchIter<'a>;

    fn to_queries(&'a self) -> Self::Queries {
        SearchIter::new(self)
    }
}

pub(crate) struct SearchIter<'a> {
    iter: [Option<(&'a str, OptionSerialize<'a>)>; 8],
    idx: usize,
}

impl<'a> SearchIter<'a> {
    pub fn new(owner: &'a Search) -> Self {
        let iter = [
            owner.jql.as_ref().map(|v| (key::JQL, v.as_str().into())),
            owner
                .start_at
                .as_ref()
                .map(|v| (key::START_AT, (*v).into())),
            owner
                .max_results
                .as_ref()
                .map(|v| (key::MAX_RESULTS, (*v).into())),
            owner
                .validate
                .as_ref()
                .map(|v| (key::VALIDATE_QUERY, (*v).into())),
            owner
                .with_fields
                .as_ref()
                .map(|v| (key::WITH_FIELDS, v.into())),
            owner.expand.as_ref().map(|v| (key::EXPAND, v.into())),
            owner
                .properties
                .as_ref()
                .map(|v| (key::PROPERTIES, v.into())),
            owner
                .fields_by_key
                .as_ref()
                .map(|v| (key::FIELDS_BY_KEY, (*v).into())),
        ];

        Self { iter, idx: 0 }
    }
}

impl<'a> Iterator for SearchIter<'a> {
    type Item = (&'a str, OptionSerialize<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = None;

        while let None = next {
            if self.idx > self.iter.len() {
                return None;
            }

            if let Some(query) = self.iter.iter_mut().nth(self.idx).and_then(|o| o.take()) {
                next = Some(query)
            }
            self.idx += 1;
        }

        next
    }
}
