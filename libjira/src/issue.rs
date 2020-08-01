pub use crate::{models::issue as models, options::issue as options};

use {
    crate::{client::Jira, error::JiraError, options::with_options},
    models::{Issue, IssueSearch},
    reqwest::{header, RequestBuilder},
};

#[derive(Debug, Clone)]
pub struct Issues {
    agent: Jira,
}

impl Issues {
    pub fn new(jira: &Jira) -> Self {
        Self {
            agent: jira.clone(),
        }
    }

    pub async fn get<K>(&self, key: K, options: Option<&options::Get>) -> Result<Issue, JiraError>
    where
        K: AsRef<str>,
    {
        let handler = |req: RequestBuilder| Ok(with_options(req, options));

        self.agent
            .get(&["issue", key.as_ref()], handler)?
            .retrieve()
            .await
    }

    pub async fn search(
        &self,
        options: Option<&options::Search>,
    ) -> Result<IssueSearch, JiraError> {
        let handler = |req| Ok(with_options(req, options));

        self.agent.get(&["search"], handler)?.retrieve().await
    }
}
