pub use crate::{models::issue as models, options::issue as options};

use {
    self::endpoint::*,
    crate::{client::Jira, error::JiraError, models::empty::Empty},
    models::{CreatedHandle, IssueHandle, MetaCreateHandle, MetaEditHandle, SearchHandle},
    reqwest::RequestBuilder,
    serde::Serialize,
};

/// A handle for interacting with JIRA issues
///
/// It is cheap to clone a handle and may be done liberally
#[derive(Debug, Clone)]
pub struct Issues {
    client: Jira,
}

impl Issues {
    /// Create a new handle from a JIRA client
    pub fn new(client: &Jira) -> Self {
        Self {
            client: client.clone(),
        }
    }

    /// Retrieve a single JIRA issue
    ///
    /// By default, this will return all available fields
    /// & field data for the issue, which can be very expensive.
    /// You may use the passed options to constrain the returned data
    pub async fn get<K>(
        &self,
        key: K,
        options: Option<&options::Get>,
    ) -> Result<IssueHandle, JiraError>
    where
        K: AsRef<str>,
    {
        let handler = |req: RequestBuilder| Ok(apply(options, req));

        self.client
            .get(&[ISSUE, key.as_ref()], handler)?
            .retrieve()
            .await
    }

    /// Search this JIRA's issues via the passed options
    ///
    /// Of special note in the passed options is the `jql`
    /// field which controls the performed search.
    ///
    /// See the following for a primer on JIRA's JQL syntax:
    /// - [What is JQL](https://support.atlassian.com/jira-software-cloud/docs/what-is-advanced-searching-in-jira-cloud)
    pub async fn search(
        &self,
        options: Option<&options::Search>,
    ) -> Result<SearchHandle, JiraError> {
        let handler = |req| Ok(apply(options, req));

        self.client.get(&[SEARCH], handler)?.retrieve().await
    }

    /// Create a new issue from a serializable struct
    ///
    /// This struct should contain at least one of:
    /// - fields: ...
    /// - update: ...
    /// In addition, it must contain all of the datums required
    /// by JIRA - `project name`, `issue type` & any `required` fields
    /// of that issue type.
    ///
    /// If your struct has both 'fields:' & 'update:' you
    /// must ensure that no field is present in both.
    ///
    /// You can use the `meta_create` and `meta_edit` methods
    /// to get the relevant information.
    ///
    /// Finally, the following links may be helpful:
    /// - [endpoint docs](https://developer.atlassian.com/cloud/jira/platform/rest/v2/api-group-issues/#api-rest-api-2-issue-post)
    /// - [fields vs update](https://developer.atlassian.com/server/jira/platform/jira-rest-api-examples)
    pub async fn create<T>(
        &self,
        issue: &T,
        options: Option<&options::Create>,
    ) -> Result<CreatedHandle, JiraError>
    where
        T: Serialize,
    {
        let handler = |req| Ok(apply(options, req).json(issue));

        self.client.post(&[ISSUE], handler)?.retrieve().await
    }

    /// Edit an existing issue with the passed serializable changes
    ///
    /// These changes should include at least one of:
    /// - fields: ...
    /// - update: ...
    ///
    /// You can use the `meta_edit` method to get the schema of this issue's
    /// changeable fields.
    pub async fn edit<K, T>(&self, key: K, changes: &T) -> Result<(), JiraError>
    where
        K: AsRef<str>,
        T: Serialize,
    {
        let handler = |req: RequestBuilder| Ok(req.json(changes));

        self.client
            .put(&[ISSUE, key.as_ref()], handler)?
            .retrieve::<Empty>()
            .await
            .map(Into::into)
    }

    /// Retrieve metadata about this JIRA's project's
    /// issue types, constrained via the passed options
    pub async fn meta_create(
        &self,
        options: Option<&options::MetaCreate>,
    ) -> Result<MetaCreateHandle, JiraError> {
        let handler = |req| Ok(apply(options, req));

        self.client
            .get(&[ISSUE, CREATE_M], handler)?
            .retrieve()
            .await
    }

    /// Retrieve metadata about a specific issue
    pub async fn meta_edit<K>(&self, key: K) -> Result<MetaEditHandle, JiraError>
    where
        K: AsRef<str>,
    {
        self.client
            .get(&[ISSUE, key.as_ref(), EDIT_M], |req| Ok(req))?
            .retrieve()
            .await
    }
}

fn apply<'a, S>(options: Option<&'a S>, req: RequestBuilder) -> RequestBuilder
where
    S: Serialize,
{
    match options {
        Some(options) => req.query(options),
        None => req,
    }
}

mod endpoint {
    pub(super) const ISSUE: &str = "issue";
    pub(super) const SEARCH: &str = "search";
    pub(super) const CREATE_M: &str = "createmeta";
    pub(super) const EDIT_M: &str = "editmeta";
}
