pub use crate::models::issue as models;
pub use crate::options::IssueOptions;

use {
    crate::{
        client::{parse_response, Jira},
        error::JiraError,
        options::with_options,
    },
    models::IssueResponse,
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

    pub async fn get<T>(
        &self,
        key: T,
        options: Option<&IssueOptions>,
    ) -> Result<IssueResponse, JiraError>
    where
        T: AsRef<str>,
    {
        let handler = |req: RequestBuilder| {
            let req = req.header(header::ACCEPT, "application/json");
            let req = with_options(req, options);

            Ok(req)
        };

        let response = self
            .agent
            .get(&["issue", key.as_ref()], handler)?
            .send()
            .await?;

        parse_response(response).await
    }
}
