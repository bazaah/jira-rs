use super::*;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Create {
    #[serde(rename = "updateHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    update_history: Option<bool>,
}

impl Create {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_history(&mut self, update: impl Into<Option<bool>>) -> &mut Self {
        self.update_history = update.into().filter(|u| *u);
        self
    }

    pub fn with<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Self) -> &mut Self,
    {
        let mut this = self;
        f(&mut this);
        this
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn empty() {
        let create = Create::new();
        let req = generate(&create);

        assert_eq!(req.url().query(), None);
    }

    #[test]
    fn single() {
        let create = Create::new().with(|this| this.update_history(true));
        let req = generate(&create);
        let query = req.url().query().expect("a non-empty query");

        assert_eq!(query, "updateHistory=true")
    }

    fn generate(s: impl Serialize) -> reqwest::Request {
        reqwest::Client::new()
            .get("http://localhost")
            .query(&s)
            .build()
            .expect("a valid request")
    }
}
