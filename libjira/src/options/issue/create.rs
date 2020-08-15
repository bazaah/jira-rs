use super::*;

#[derive(Debug, Default, Clone)]
pub struct Create {
    update_history: Option<bool>,
}

impl Create {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_history(self, update: Option<bool>) -> Self {
        let mut this = self;
        this.update_history = update.filter(|b| *b);
        this
    }
}

impl<'a> ToQuery<'a> for Create {
    type Queries = CreateIter<'a>;

    fn to_queries(&'a self) -> Self::Queries {
        CreateIter::new(self)
    }
}

#[derive(Debug)]
pub(crate) struct CreateIter<'a> {
    iter: Option<(&'a str, OptionSerialize<'a>)>,
    idx: bool,
}

impl<'a> CreateIter<'a> {
    pub fn new(owner: &'a Create) -> Self {
        let iter = owner
            .update_history
            .map(|v| (key::UPDATE_HISTORY, v.into()));

        Self { iter, idx: false }
    }
}

impl<'a> Iterator for CreateIter<'a> {
    type Item = (&'a str, OptionSerialize<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx {
            return None;
        }
        self.idx = true;

        self.iter.take()
    }
}
