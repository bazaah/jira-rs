use {
    json::{value::RawValue as RawJson, Error as JsonError},
    serde::Deserialize,
    serde_json as json,
    std::collections::HashMap,
};

#[derive(Debug, Clone, Deserialize)]
pub struct IssueSearch {
    pub expand: String,
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    #[serde(rename = "startAt")]
    pub start_at: u64,
    pub total: u64,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
    #[serde(rename = "self")]
    pub self_link: String,
    pub id: String,
    pub key: String,
    pub expand: String,
    pub fields: HashMap<String, Box<RawJson>>,

    // Capture any extra fields returned
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

    // User assigned to the issue
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

    // Issue's resolution date
    pub fn resolution_date(&self) -> Option<&str> {
        self.string_field("resolutiondate").and_then(Result::ok)
    }

    pub fn issue_type(&self) -> Option<IssueType> {
        self.field::<IssueType>("issuetype").and_then(Result::ok)
    }

    pub fn labels(&self) -> Option<Vec<&str>> {
        self.field::<Vec<&str>>("labels").and_then(Result::ok)
    }

    pub fn fix_versions(&self) -> Option<Vec<&str>> {
        self.field::<Vec<&str>>("fixVersions").and_then(Result::ok)
    }

    pub fn comments(&self) -> Option<Vec<Comment>> {
        // Note JIRA's json path here looks like: issue.comment.comments.[ <-- Comment objects here --> ]
        // We remove some of this indirection here, so it appears to the user like: issue.comments.[...]
        self.field::<Comments>("comment")
            .and_then(|r| r.map(Into::into).ok())
    }

    /// Issue priority
    pub fn priority(&self) -> Option<Priority> {
        self.field::<Priority>("priority").and_then(Result::ok)
    }

    /// Other Issues that are linked to the current Issue
    pub fn issue_links(&self) -> Option<Vec<IssueLink>> {
        self.field::<Vec<IssueLink>>("issuelinks")
            .and_then(Result::ok)
    }

    pub fn project(&self) -> Option<Project> {
        self.field::<Project>("project").and_then(Result::ok)
    }

    pub fn resolution(&self) -> Option<Resolution> {
        self.field::<Resolution>("resolution").and_then(Result::ok)
    }

    pub fn attachment(&self) -> Option<Vec<Attachment>> {
        self.field::<Vec<Attachment>>("attachment")
            .and_then(Result::ok)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct User<'a> {
    pub active: bool,
    #[serde(rename = "avatarUrls")]
    pub avatar_urls: HashMap<&'a str, &'a str>,
    #[serde(rename = "displayName")]
    pub display_name: &'a str,
    #[serde(rename = "emailAddress")]
    pub email_address: &'a str,
    pub key: Option<&'a str>,
    pub name: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
    #[serde(rename = "timeZone")]
    pub timezone: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Status<'a> {
    pub description: &'a str,
    #[serde(rename = "iconUrl")]
    pub icon_url: &'a str,
    pub id: &'a str,
    pub name: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueType<'a> {
    pub description: &'a str,
    #[serde(rename = "iconUrl")]
    pub icon_url: &'a str,
    pub id: &'a str,
    pub name: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
    pub subtask: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Version<'a> {
    pub archived: bool,
    pub id: &'a str,
    pub name: &'a str,
    pub released: bool,
    #[serde(rename = "self")]
    pub self_link: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
struct Comments<'a> {
    #[serde(borrow)]
    inner: Vec<Comment<'a>>,
}

impl<'a> Into<Vec<Comment<'a>>> for Comments<'a> {
    fn into(self) -> Vec<Comment<'a>> {
        self.inner
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Comment<'a> {
    pub id: Option<&'a str>,
    #[serde(rename = "self")]
    pub self_link: &'a str,
    pub author: Option<User<'a>>,
    #[serde(rename = "updateAuthor")]
    pub update_author: Option<User<'a>>,
    pub created: &'a str,
    pub updated: &'a str,
    pub body: &'a str,
    pub visibility: Option<Visibility<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Visibility<'a> {
    #[serde(rename = "type")]
    pub visibility_type: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Project<'a> {
    pub id: &'a str,
    pub key: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IssueLink<'a> {
    pub id: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
    #[serde(rename = "outwardIssue")]
    pub outward_issue: Option<Issue>,
    #[serde(rename = "inwardIssue")]
    pub inward_issue: Option<Issue>,
    #[serde(rename = "type")]
    pub link_type: LinkType<'a>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LinkType<'a> {
    pub id: &'a str,
    pub inward: &'a str,
    pub name: &'a str,
    pub outward: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Resolution<'a> {
    name: &'a str,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Attachment<'a> {
    pub id: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
    pub filename: &'a str,
    pub author: User<'a>,
    pub created: &'a str,
    pub size: u64,
    #[serde(rename = "mimeType")]
    pub mime_type: &'a str,
    pub content: &'a str,
    pub thumbnail: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Priority<'a> {
    pub icon_url: &'a str,
    pub id: &'a str,
    pub name: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
}
