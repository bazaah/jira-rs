pub use crate::{models::issue as models, options::issue as options};

use {
    self::endpoint::*,
    crate::{client::Jira, error::JiraError, options::ToQuery},
    models::{Issue, MetaCreate, Search},
    reqwest::RequestBuilder,
};

#[derive(Debug, Clone)]
pub struct Issues {
    client: Jira,
}

impl Issues {
    pub fn new(client: &Jira) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn get<K>(
        &self,
        key: K,
        options: Option<&options::Get<'_>>,
    ) -> Result<Issue, JiraError>
    where
        K: AsRef<str>,
    {
        let handler = |req: RequestBuilder| Ok(apply(options, req));

        self.client
            .get(&[ISSUE, key.as_ref()], handler)?
            .retrieve()
            .await
    }

    pub async fn search(&self, options: Option<&options::Search<'_>>) -> Result<Search, JiraError> {
        let handler = |req| Ok(apply(options, req));

        self.client.get(&[SEARCH], handler)?.retrieve().await
    }

    pub async fn meta_create(&self) -> Result<MetaCreate, JiraError> {
        let handler = |req| Ok(req);

        self.client
            .get(&[ISSUE, CREATE_M], handler)?
            .retrieve()
            .await
    }
}

fn apply<'a, O>(options: Option<&'a O>, req: RequestBuilder) -> RequestBuilder
where
    O: ToQuery<'a>,
{
    match options {
        Some(options) => options.append_request(req),
        None => req,
    }
}

mod endpoint {
    pub(super) const ISSUE: &'static str = "issue";
    pub(super) const SEARCH: &'static str = "search";
    pub(super) const CREATE_M: &'static str = "createmeta";
}
