use super::*;

pub use super::ValidateQuery;

#[derive(Debug, Default, Clone)]
pub struct Get {
    with_fields: Option<CommaDelimited>,
    expand: Option<CommaDelimited>,
    fields_by_key: Option<bool>,
    properties: Option<CommaDelimited>,
    update_history: Option<bool>,
}

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

#[derive(Debug, Default, Clone)]
pub struct MetaCreate {
    project_ids: Option<CommaDelimited>,
    project_keys: Option<CommaDelimited>,
    issuetype_ids: Option<CommaDelimited>,
    issuetype_keys: Option<CommaDelimited>,
    expand: Option<CommaDelimited>,
}

impl Get {
    pub fn new() -> Self {
        Self::default()
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

    pub fn update_history(self, update: Option<bool>) -> Self {
        let mut this = self;
        this.update_history = update.filter(|b| *b);
        this
    }
}

impl<'a> ToQuery<'a> for Get {
    type Queries = GetIter<'a>;

    fn to_queries(&'a self) -> Self::Queries {
        GetIter::new(self)
    }
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

impl MetaCreate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn project_keys<'a, I>(self, project_keys: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        let mut this = self;
        this.project_keys = project_keys.map(|i| CommaDelimited::from_iter(i));
        this
    }

    pub fn project_ids<I>(self, project_ids: Option<I>) -> Self
    where
        I: Iterator<Item = u64> + Clone,
    {
        let mut this = self;
        this.project_ids = project_ids.map(|i| CommaDelimited::from_iter(i));
        this
    }

    pub fn issuetype_keys<'a, I>(self, issuetype_keys: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        let mut this = self;
        this.issuetype_keys = issuetype_keys.map(|i| CommaDelimited::from_iter(i));
        this
    }

    pub fn issuetype_ids<I>(self, issuetype_ids: Option<I>) -> Self
    where
        I: Iterator<Item = u64> + Clone,
    {
        let mut this = self;
        this.issuetype_ids = issuetype_ids.map(|i| CommaDelimited::from_iter(i));
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
}

impl<'a> ToQuery<'a> for MetaCreate {
    type Queries = MetaCreateIter<'a>;

    fn to_queries(&'a self) -> Self::Queries {
        MetaCreateIter::new(self)
    }
}

/*
 <=========== OPTION ITERATORS ========================>
*/

pub(crate) struct GetIter<'a> {
    iter: [Option<(&'a str, OptionSerialize<'a>)>; 5],
    idx: usize,
}

impl<'a> GetIter<'a> {
    pub fn new(owner: &'a Get) -> Self {
        let iter = [
            owner
                .with_fields
                .as_ref()
                .map(|v| (key::WITH_FIELDS, v.into())),
            owner.expand.as_ref().map(|v| (key::EXPAND, v.into())),
            owner
                .fields_by_key
                .as_ref()
                .map(|v| (key::FIELDS_BY_KEY, (*v).into())),
            owner
                .properties
                .as_ref()
                .map(|v| (key::PROPERTIES, v.into())),
            owner
                .update_history
                .as_ref()
                .map(|v| (key::UPDATE_HISTORY, (*v).into())),
        ];

        Self { iter, idx: 0 }
    }
}

impl<'a> Iterator for GetIter<'a> {
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

pub(crate) struct MetaCreateIter<'a> {
    iter: [Option<(&'a str, OptionSerialize<'a>)>; 5],
    idx: usize,
}

impl<'a> MetaCreateIter<'a> {
    pub fn new(owner: &'a MetaCreate) -> Self {
        let iter = [
            owner
                .project_ids
                .as_ref()
                .map(|v| (key::PROJECT_IDS, v.into())),
            owner
                .project_keys
                .as_ref()
                .map(|v| (key::PROJECT_KEYS, v.into())),
            owner
                .issuetype_ids
                .as_ref()
                .map(|v| (key::ISSUETYPE_IDS, v.into())),
            owner
                .issuetype_keys
                .as_ref()
                .map(|v| (key::ISSUETYPE_KEYS, v.into())),
            owner.expand.as_ref().map(|v| (key::EXPAND, v.into())),
        ];

        Self { iter, idx: 0 }
    }
}

impl<'a> Iterator for MetaCreateIter<'a> {
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
