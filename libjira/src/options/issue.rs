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

#[derive(Debug, Default, Clone)]
pub struct MetaCreate<'a> {
    project_ids: Option<Vec<u64>>,
    project_keys: Option<Vec<SmolCow<'a, str>>>,
    issuetype_ids: Option<Vec<u64>>,
    issuetype_keys: Option<Vec<SmolCow<'a, str>>>,
}

impl Get<'static> {
    pub fn into_owned<'a>(get: Get<'a>) -> Self {
        Get {
            with_fields: get.with_fields.map(|v| {
                v.into_iter()
                    .map(|s| SmolCow::Owned(s.to_owned()))
                    .collect()
            }),
            expand: get.expand.map(|v| {
                v.into_iter()
                    .map(|s| SmolCow::Owned(s.to_owned()))
                    .collect()
            }),
            fields_by_key: get.fields_by_key,
            properties: get.properties.map(|v| {
                v.into_iter()
                    .map(|s| SmolCow::Owned(s.to_owned()))
                    .collect()
            }),
            update_history: get.update_history,
        }
    }
}

impl<'a> Get<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_fields<I>(self, fields: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut this = self;
        this.with_fields = fields.map(|i| i.map(|s| s.into()).collect());
        this
    }

    pub fn expand<I>(self, expand: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut this = self;
        this.expand = expand.map(|i| i.map(|s| s.into()).collect());
        this
    }

    pub fn properties<I>(self, properties: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut this = self;
        this.properties = properties.map(|i| i.map(|s| s.into()).collect());
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

impl<'a> ToQuery<'a> for Get<'a> {
    type Queries = GetIter<'a>;

    fn to_queries(&'a self) -> Self::Queries {
        GetIter::new(self)
    }
}

impl Search<'static> {
    pub fn into_owned<'a>(search: Search<'a>) -> Self {
        Search {
            jql: search.jql.map(|s| SmolCow::Owned(s.to_owned())),
            start_at: search.start_at,
            max_results: search.max_results,
            validate: search.validate,
            with_fields: search.with_fields.map(|v| {
                v.into_iter()
                    .map(|s| SmolCow::Owned(s.to_owned()))
                    .collect()
            }),
            expand: search.expand.map(|v| {
                v.into_iter()
                    .map(|s| SmolCow::Owned(s.to_owned()))
                    .collect()
            }),
            fields_by_key: search.fields_by_key,
            properties: search.properties.map(|v| {
                v.into_iter()
                    .map(|s| SmolCow::Owned(s.to_owned()))
                    .collect()
            }),
        }
    }
}

impl<'a> Search<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn jql<T: 'a>(self, jql: Option<T>) -> Self
    where
        T: Into<SmolCow<'a, str>>,
    {
        let mut this = self;
        this.jql = jql.map(|s| s.into());
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

    pub fn with_fields<I>(self, fields: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut this = self;
        this.with_fields = fields.map(|i| i.map(|s| s.into()).collect());
        this
    }

    pub fn expand<I>(self, expand: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut this = self;
        this.expand = expand.map(|i| i.map(|s| s.into()).collect());
        this
    }

    pub fn properties<I>(self, properties: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut this = self;
        this.properties = properties.map(|i| i.map(|s| s.into()).collect());
        this
    }

    pub fn fields_by_key(self, by_key: Option<bool>) -> Self {
        let mut this = self;
        this.fields_by_key = by_key.filter(|b| *b);
        this
    }
}

impl<'a> ToQuery<'a> for Search<'a> {
    type Queries = SearchIter<'a>;

    fn to_queries(&'a self) -> Self::Queries {
        SearchIter::new(self)
    }
}

impl<'a> MetaCreate<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn project_keys<I>(self, project_keys: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut this = self;
        this.project_keys = project_keys.map(|i| i.map(|s| s.into()).collect());
        this
    }

    pub fn project_ids<I>(self, project_ids: Option<I>) -> Self
    where
        I: Iterator<Item = u64>,
    {
        let mut this = self;
        this.project_ids = project_ids.map(|i| i.collect());
        this
    }

    pub fn issuetype_keys<I>(self, issuetype_keys: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut this = self;
        this.issuetype_keys = issuetype_keys.map(|i| i.map(|s| s.into()).collect());
        this
    }

    pub fn issuetype_ids<I>(self, issuetype_ids: Option<I>) -> Self
    where
        I: Iterator<Item = u64>,
    {
        let mut this = self;
        this.issuetype_ids = issuetype_ids.map(|i| i.collect());
        this
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
    pub fn new(owner: &'a Get<'a>) -> Self {
        let iter = [
            owner
                .with_fields
                .as_ref()
                .map(|v| (key::WITH_FIELDS, CommaDelimited::new(v).into())),
            owner
                .expand
                .as_ref()
                .map(|v| (key::EXPAND, CommaDelimited::new(v).into())),
            owner
                .fields_by_key
                .as_ref()
                .map(|v| (key::FIELDS_BY_KEY, (*v).into())),
            owner
                .properties
                .as_ref()
                .map(|v| (key::PROPERTIES, CommaDelimited::new(v).into())),
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
    pub fn new(owner: &'a Search<'a>) -> Self {
        let iter = [
            owner.jql.as_ref().map(|v| (key::JQL, v.as_ref().into())),
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
                .map(|v| (key::WITH_FIELDS, CommaDelimited::new(v).into())),
            owner
                .expand
                .as_ref()
                .map(|v| (key::EXPAND, CommaDelimited::new(v).into())),
            owner
                .properties
                .as_ref()
                .map(|v| (key::PROPERTIES, CommaDelimited::new(v).into())),
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

//pub(crate) struct MetaCreateIter<'a> {
//    iter: [Option<(&'a str, OptionSerialize<'a>)>; 4],
//    idx: usize,
//}
//
//impl<'a> MetaCreateIter<'a> {
//    pub fn new(owner: &'a MetaCreate<'a>) -> Self {
//        let iter = [
//            owner
//                .with_fields
//                .as_ref()
//                .map(|v| (key::WITH_FIELDS, CommaDelimited::new(v).into())),
//            owner
//                .expand
//                .as_ref()
//                .map(|v| (key::EXPAND, CommaDelimited::new(v).into())),
//            owner
//                .fields_by_key
//                .as_ref()
//                .map(|v| (key::FIELDS_BY_KEY, (*v).into())),
//            owner
//                .properties
//                .as_ref()
//                .map(|v| (key::PROPERTIES, CommaDelimited::new(v).into())),
//            owner
//                .update_history
//                .as_ref()
//                .map(|v| (key::UPDATE_HISTORY, (*v).into())),
//        ];
//
//        Self { iter, idx: 0 }
//    }
//}
//
//impl<'a> Iterator for MetaCreateIter<'a> {
//    type Item = (&'a str, OptionSerialize<'a>);
//
//    fn next(&mut self) -> Option<Self::Item> {
//        let mut next = None;
//
//        while let None = next {
//            if self.idx > self.iter.len() {
//                return None;
//            }
//
//            if let Some(query) = self.iter.iter_mut().nth(self.idx).and_then(|o| o.take()) {
//                next = Some(query)
//            }
//            self.idx += 1;
//        }
//
//        next
//    }
//}
