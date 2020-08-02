pub use crate::{models::issue as models, options::issue as options};

use {
    crate::{client::Jira, error::JiraError, options::with_options},
    models::{Issue, Search},
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
        let handler = |req: RequestBuilder| Ok(with_options(req, options));

        self.client
            .get(&["issue", key.as_ref()], handler)?
            .retrieve()
            .await
    }

    pub async fn search(&self, options: Option<&options::Search>) -> Result<Search, JiraError> {
        let handler = |req| Ok(with_options(req, options));

        self.client.get(&["search"], handler)?.retrieve().await
    }
}
