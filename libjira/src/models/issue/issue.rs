use {
    super::*,
    json::{value::RawValue as RawJson, Error as JsonError},
    serde::{Deserialize, Serialize},
    serde_json as json,
    std::collections::HashMap,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Issue {
    #[serde(rename = "self")]
    pub self_link: String,
    pub id: String,
    pub key: String,
    pub expand: String,
    pub fields: HashMap<String, Box<RawJson>>,

    // Capture any extra fields returned (if any)
    #[serde(flatten)]
    pub extra: HashMap<String, Box<RawJson>>,
}

impl Issue {
    /// Attempt to deserialize an arbitrary value with the given key from the `fields` of this Issue.
    ///
    /// Note the bound `T: Deserialize<'de>` allows for zero copy deserialization,
    /// with the lifetime tied to this `Issue`
    pub fn field<'de, 'a: 'de, T>(&'a self, key: &str) -> Option<Result<T, JsonError>>
    where
        T: Deserialize<'de>,
    {
        self.fields.get(key).map(|raw| json::from_str(raw.get()))
    }

    /// Attempt to deserialize an arbitrary value with the given key from the `extra` fields of this Issue.
    ///
    /// Note the bound `T: Deserialize<'de>` allows for zero copy deserialization,
    /// with the lifetime tied to this `Issue`
    pub fn extra<'de, 'a: 'de, T>(&'a self, key: &str) -> Option<Result<T, JsonError>>
    where
        T: Deserialize<'de>,
    {
        self.extra.get(key).map(|raw| json::from_str(raw.get()))
    }

    fn string_field<'a>(&'a self, key: &str) -> Option<Result<&str, JsonError>> {
        self.field::<&str>(key)
    }

    fn user_field<'a>(&'a self, key: &str) -> Option<Result<User, JsonError>> {
        self.field::<User>(key)
    }

    /// User assigned to the issue
    pub fn assignee(&self) -> Option<User> {
        self.field("assignee").and_then(Result::ok)
    }

    /// User that originally created the issue
    pub fn creator(&self) -> Option<User> {
        self.user_field("creator").and_then(Result::ok)
    }

    /// User that reported the issue
    pub fn reporter(&self) -> Option<User> {
        self.user_field("reporter").and_then(Result::ok)
    }

    /// Issue summary
    pub fn summary(&self) -> Option<&str> {
        self.string_field("summary").and_then(Result::ok)
    }

    /// Issue status
    pub fn status(&self) -> Option<Status> {
        self.field::<Status>("status").and_then(Result::ok)
    }

    /// Issue description
    pub fn description(&self) -> Option<&str> {
        self.string_field("description").and_then(Result::ok)
    }

    /// Issue's latest update timestamp
    pub fn updated(&self) -> Option<&str> {
        self.string_field("updated").and_then(Result::ok)
    }

    /// Issue's creation timestamp
    pub fn created(&self) -> Option<&str> {
        self.string_field("created").and_then(Result::ok)
    }

    /// Issue's resolution date
    pub fn resolution_date(&self) -> Option<&str> {
        self.string_field("resolutiondate").and_then(Result::ok)
    }

    /// Description of the issue's type
    pub fn issue_type(&self) -> Option<IssueType> {
        self.field::<IssueType>("issuetype").and_then(Result::ok)
    }

    /// Labels assigned to this issue
    pub fn labels(&self) -> Option<Vec<&str>> {
        self.field::<Vec<&str>>("labels").and_then(Result::ok)
    }

    // TODO: This appears to return an object not str... investigate
    /// Issue fix version(s)
    pub fn fix_versions(&self) -> Option<Vec<&str>> {
        self.field::<Vec<&str>>("fixVersions").and_then(Result::ok)
    }

    /// Issue's comments
    pub fn comments(&self) -> Option<Vec<Comment>> {
        // Note JIRA's json path here looks like: issue.comment.comments.[ <-- Comment objects here --> ]
        // We remove some of this indirection here, so it appears to the user like: issue.comments.[...]
        self.field::<Comments>("comment")
            .and_then(|r| r.map(Into::into).ok())
    }

    /// Issue's priority
    pub fn priority(&self) -> Option<Priority> {
        self.field::<Priority>("priority").and_then(Result::ok)
    }

    /// Other Issues that are linked to the current Issue
    pub fn issue_links(&self) -> Option<Vec<IssueLink>> {
        self.field::<Vec<IssueLink>>("issuelinks")
            .and_then(Result::ok)
    }

    /// The project this Issue is assigned to
    pub fn project(&self) -> Option<Project> {
        self.field::<Project>("project").and_then(Result::ok)
    }

    /// This Issue's resolution, if it exists
    pub fn resolution(&self) -> Option<Resolution> {
        self.field::<Resolution>("resolution").and_then(Result::ok)
    }

    /// Any attachments this Issue contains
    pub fn attachment(&self) -> Option<Vec<Attachment>> {
        self.field::<Vec<Attachment>>("attachment")
            .and_then(Result::ok)
    }
}

