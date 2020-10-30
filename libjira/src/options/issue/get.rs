use super::*;

#[derive(Debug, Default, Clone)]
pub struct Get {
    with_fields: Option<CommaDelimited>,
    expand: Option<CommaDelimited>,
    fields_by_key: Option<bool>,
    properties: Option<CommaDelimited>,
    update_history: Option<bool>,
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
        if let Some(value) = fields {
            this.with_fields = this
                .with_fields
                .unwrap_or_default()
                .with(|s| s.from_iter(value))
                .into();
        }
        this
    }

    pub fn expand<'a, I>(self, expand: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        let mut this = self;
        if let Some(value) = expand {
            this.expand = this
                .expand
                .unwrap_or_default()
                .with(|s| s.from_iter(value))
                .into();
        }
        this
    }

    pub fn properties<'a, I>(self, properties: Option<I>) -> Self
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        let mut this = self;
        if let Some(value) = properties {
            this.properties = this
                .properties
                .unwrap_or_default()
                .with(|s| s.from_iter(value))
                .into();
        }
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

#[derive(Debug)]
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

