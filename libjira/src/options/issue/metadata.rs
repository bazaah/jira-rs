use super::*;

#[derive(Debug, Default, Clone)]
pub struct MetaCreate {
    project_ids: Option<CommaDelimited>,
    project_keys: Option<CommaDelimited>,
    issuetype_ids: Option<CommaDelimited>,
    issuetype_keys: Option<CommaDelimited>,
    expand: Option<CommaDelimited>,
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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    const VALS: &'static [&'static str; 4] = &["ONE", "__$@|`", "P34", "lowercase"];
    const IDS: &'static [u64; 4] = &[1, 00002, 345675, 4];

    #[test]
    fn none() {
        let MetaCreate {
            project_ids,
            project_keys,
            issuetype_ids,
            issuetype_keys,
            expand,
        } = MetaCreate::new();

        assert!(project_ids.is_none());
        assert!(project_keys.is_none());
        assert!(issuetype_ids.is_none());
        assert!(issuetype_keys.is_none());
        assert!(expand.is_none());
    }

    #[test]
    fn single() {
        let MetaCreate {
            project_ids,
            project_keys,
            issuetype_ids,
            issuetype_keys,
            expand,
        } = generate(1);

        equals(project_ids, "1");
        equals(issuetype_ids, "1");
        equals(project_keys, "ONE");
        equals(issuetype_keys, "ONE");
        equals(expand, "ONE");
    }

    #[test]
    fn multiple() {
        let MetaCreate {
            project_ids,
            project_keys,
            issuetype_ids,
            issuetype_keys,
            expand,
        } = generate(3);

        equals(project_ids, "1,2,345675");
        equals(issuetype_ids, "1,2,345675");
        equals(project_keys, "ONE,__$@|`,P34");
        equals(issuetype_keys, "ONE,__$@|`,P34");
        equals(expand, "ONE,__$@|`,P34");
    }

    fn generate(range: usize) -> MetaCreate {
        MetaCreate::new()
            .issuetype_ids(u_range(range))
            .issuetype_keys(s_range(range))
            .project_keys(s_range(range))
            .project_ids(u_range(range))
            .expand(s_range(range))
    }

    fn equals(o: Option<CommaDelimited>, val: &str) {
        assert_eq!(o.expect("A valid comma delimited value").as_ref(), val)
    }

    fn u_range(range: usize) -> Option<impl Iterator<Item = u64> + Clone> {
        Some(IDS).map(|v| v.iter().take(range).copied())
    }

    fn s_range(range: usize) -> Option<impl Iterator<Item = &'static str> + Clone> {
        Some(VALS).map(|v| v.iter().take(range).copied())
    }
}
