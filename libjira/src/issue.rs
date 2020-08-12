pub use crate::{models::issue as models, options::issue as options};

use {
    self::endpoint::*,
    crate::{client::Jira, error::JiraError, options::ToQuery},
    models::{Issue, MetaCreate, MetaEdit, Search},
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

    pub async fn get<K>(&self, key: K, options: Option<&options::Get>) -> Result<Issue, JiraError>
    where
        K: AsRef<str>,
    {
        let handler = |req: RequestBuilder| Ok(apply(options, req));

        self.client
            .get(&[ISSUE, key.as_ref()], handler)?
            .retrieve()
            .await
    }

    pub async fn search(&self, options: Option<&options::Search>) -> Result<Search, JiraError> {
        let handler = |req| Ok(apply(options, req));

        self.client.get(&[SEARCH], handler)?.retrieve().await
    }

    pub async fn meta_create(
        &self,
        options: Option<&options::MetaCreate>,
    ) -> Result<MetaCreate, JiraError> {
        let handler = |req| Ok(apply(options, req));

        self.client
            .get(&[ISSUE, CREATE_M], handler)?
            .retrieve()
            .await
    }

    pub async fn meta_edit<K>(&self, key: K) -> Result<MetaEdit, JiraError>
    where
        K: AsRef<str>,
    {
        self.client
            .get(&[ISSUE, key.as_ref(), EDIT_M], |req| Ok(req))?
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
    pub(super) const EDIT_M: &'static str = "editmeta";
}
