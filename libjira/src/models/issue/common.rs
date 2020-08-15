use {super::*, serde::de::Deserializer, std::collections::HashMap};

/// Representation of a Jira User
#[derive(Debug, Clone, Deserialize, Serialize)]
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

/// Representation of the current status of
/// the issue, examples include: "In Progress", "Testing", "Done"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status<'a> {
    pub description: &'a str,
    #[serde(rename = "iconUrl")]
    pub icon_url: &'a str,
    pub id: &'a str,
    pub name: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
    // TODO: Add statusCategory as optional
}

/// The issue kind, examples include: "Story", "Epic"
#[derive(Debug, Clone, Deserialize, Serialize)]
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

// TODO: Investigate where this shows up
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Version<'a> {
    pub archived: bool,
    pub id: &'a str,
    pub name: &'a str,
    pub released: bool,
    #[serde(rename = "self")]
    pub self_link: &'a str,
}

// Wrapper struct for flattening Jira's json
// path to comments
#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Comments<'a> {
    #[serde(borrow)]
    inner: Vec<Comment<'a>>,
}

impl<'a> Into<Vec<Comment<'a>>> for Comments<'a> {
    fn into(self) -> Vec<Comment<'a>> {
        self.inner
    }
}

/// Representation of a Jira comment
#[derive(Debug, Clone, Deserialize, Serialize)]
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

// Not all Jira's have this...
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Visibility<'a> {
    #[serde(rename = "type")]
    pub visibility_type: &'a str,
    pub value: &'a str,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project<'a> {
    pub id: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
    pub key: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinkType<'a> {
    pub id: &'a str,
    pub inward: &'a str,
    pub name: &'a str,
    pub outward: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resolution<'a> {
    name: &'a str,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Priority<'a> {
    pub icon_url: &'a str,
    pub id: &'a str,
    pub name: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
}

/// One of the flavours of response returned by some
/// endpoints. Nests the errors inside a sub-object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NestedResponse<'a> {
    pub status: u64,
    #[serde(
        borrow,
        rename = "errorCollection",
        deserialize_with = "skip_errors",
        skip_serializing_if = "Option::is_none"
    )]
    pub errors: Option<ErrorCollection<'a>>,
}

fn skip_errors<'a, 'de: 'a, D>(deserializer: D) -> Result<Option<ErrorCollection<'a>>, D::Error>
where
    D: Deserializer<'de>,
{
    let collection: ErrorCollection = Deserialize::deserialize(deserializer)?;

    match collection.is_error() {
        true => Ok(Some(collection)),
        false => Ok(None),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorCollection<'a> {
    #[serde(rename = "errorMessages", borrow)]
    pub messages: Vec<&'a str>,
    pub errors: HashMap<&'a str, &'a str>,
}

impl<'a> ErrorCollection<'a> {
    pub fn is_error(&self) -> bool {
        !(self.errors.is_empty() && self.messages.is_empty())
    }
}
