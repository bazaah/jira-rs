use {
    super::*,
    serde::{de, ser::Serializer},
    std::collections::HashMap,
    std::fmt,
};

/// Representation of a Jira User
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User<'a> {
    pub active: bool,

    /// A map of dimension keys with URL self link values.
    ///
    /// Each key is typically in the form "<INT>x<INT>", e.g
    /// "16x16", "32x32", etc.
    #[serde(
        rename = "avatarUrls",
        borrow,
        deserialize_with = "cow::deserialize_kv"
    )]
    pub avatar_urls: HashMap<Cow<'a, str>, Cow<'a, str>>,
    #[serde(rename = "displayName", borrow, deserialize_with = "cow::deserialize")]
    pub display_name: Cow<'a, str>,
    #[serde(rename = "emailAddress", borrow, deserialize_with = "cow::deserialize")]
    pub email_address: Cow<'a, str>,
    #[serde(default, borrow, deserialize_with = "cow::deserialize_option")]
    pub key: Option<Cow<'a, str>>,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub name: Cow<'a, str>,
    #[serde(rename = "self", borrow, deserialize_with = "cow::deserialize")]
    pub self_link: Cow<'a, str>,

    /// Timezone of the user, in ISO standard <COUNTRY>/<ZONE-OR-CITY> format
    #[serde(
        rename = "timeZone",
        default,
        borrow,
        deserialize_with = "cow::deserialize_option"
    )]
    pub timezone: Option<Cow<'a, str>>,
}

/// Representation of the current status of
/// the issue, examples include: "In Progress", "Testing", "Done"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status<'a> {
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub description: Cow<'a, str>,
    #[serde(rename = "iconUrl", borrow, deserialize_with = "cow::deserialize")]
    pub icon_url: Cow<'a, str>,
    #[serde(with = "id")]
    pub id: u64,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub name: Cow<'a, str>,
    #[serde(rename = "self", borrow, deserialize_with = "cow::deserialize")]
    pub self_link: Cow<'a, str>,
    // TODO: Add statusCategory as optional
}

/// The issue kind, examples include: "Story", "Epic"
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueType<'a> {
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub description: Cow<'a, str>,
    #[serde(rename = "iconUrl", borrow, deserialize_with = "cow::deserialize")]
    pub icon_url: Cow<'a, str>,
    #[serde(with = "id")]
    pub id: u64,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub name: Cow<'a, str>,
    #[serde(rename = "self", borrow, deserialize_with = "cow::deserialize")]
    pub self_link: Cow<'a, str>,
    pub subtask: bool,
}

/// A project or product release
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Version<'a> {
    pub archived: bool,
    #[serde(with = "id")]
    pub id: u64,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub name: Cow<'a, str>,
    pub released: bool,
    #[serde(rename = "self", borrow, deserialize_with = "cow::deserialize")]
    pub self_link: Cow<'a, str>,
}

// Wrapper struct for flattening Jira's json
// path to comments
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
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
    #[serde(with = "id")]
    pub id: u64,
    #[serde(rename = "self", borrow, deserialize_with = "cow::deserialize")]
    pub self_link: Cow<'a, str>,

    /// The original author of the comment
    pub author: Option<User<'a>>,

    /// The author of the update to this comment (if any)
    #[serde(rename = "updateAuthor")]
    pub update_author: Option<User<'a>>,

    /// A ISO-8601 timestamp of issue creation
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub created: Cow<'a, str>,

    /// A ISO-8601 timestamp of the latest update
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub updated: Cow<'a, str>,

    /// The comment's text
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub body: Cow<'a, str>,

    /// The visibility of the comment, if any is set
    pub visibility: Option<Visibility<'a>>,
}

// Not all Jira's have this...
/// The visibility of the associated object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Visibility<'a> {
    #[serde(rename = "type", borrow, deserialize_with = "cow::deserialize")]
    pub visibility_type: Cow<'a, str>,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub value: Cow<'a, str>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project<'a> {
    #[serde(with = "id")]
    pub id: u64,
    #[serde(rename = "self", borrow, deserialize_with = "cow::deserialize")]
    pub self_link: Cow<'a, str>,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub key: Cow<'a, str>,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub name: Cow<'a, str>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueLink<'a> {
    #[serde(with = "id")]
    pub id: u64,
    #[serde(rename = "self", borrow, deserialize_with = "cow::deserialize")]
    pub self_link: Cow<'a, str>,
    #[serde(rename = "outwardIssue")]
    pub outward_issue: Option<Issue<'a>>,
    #[serde(rename = "inwardIssue")]
    pub inward_issue: Option<Issue<'a>>,
    #[serde(rename = "type")]
    pub link_type: LinkType<'a>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinkType<'a> {
    #[serde(with = "id")]
    pub id: u64,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub inward: Cow<'a, str>,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub name: Cow<'a, str>,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub outward: Cow<'a, str>,
    #[serde(rename = "self", borrow, deserialize_with = "cow::deserialize")]
    pub self_link: Cow<'a, str>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resolution<'a> {
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    name: Cow<'a, str>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attachment<'a> {
    #[serde(with = "id")]
    pub id: u64,
    #[serde(rename = "self", borrow, deserialize_with = "cow::deserialize")]
    pub self_link: Cow<'a, str>,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub filename: Cow<'a, str>,
    pub author: User<'a>,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub created: Cow<'a, str>,
    pub size: u64,
    #[serde(rename = "mimeType", borrow, deserialize_with = "cow::deserialize")]
    pub mime_type: Cow<'a, str>,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub content: Cow<'a, str>,
    #[serde(default, borrow, deserialize_with = "cow::deserialize_option")]
    pub thumbnail: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Priority<'a> {
    #[serde(rename = "iconUrl", borrow, deserialize_with = "cow::deserialize")]
    pub icon_url: Cow<'a, str>,
    #[serde(with = "id")]
    pub id: u64,
    #[serde(borrow, deserialize_with = "cow::deserialize")]
    pub name: Cow<'a, str>,
    #[serde(rename = "self", borrow, deserialize_with = "cow::deserialize")]
    pub self_link: Cow<'a, str>,
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
    D: de::Deserializer<'de>,
{
    let collection: Option<ErrorCollection> = Deserialize::deserialize(deserializer)?;

    match collection {
        Some(col) if col.is_error() => Ok(Some(col)),
        _ => Ok(None),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ErrorCollection<'a> {
    #[serde(rename = "errorMessages", borrow)]
    pub messages: Vec<Cow<'a, str>>,
    #[serde(borrow, deserialize_with = "cow::deserialize_kv")]
    pub errors: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'a> ErrorCollection<'a> {
    pub fn is_error(&self) -> bool {
        !(self.errors.is_empty() && self.messages.is_empty())
    }
}

/// Use this module as a 'serde with' attribute on JIRA id fields
/// to correctly de/serialize the JSON string representations as `u64`s
pub mod id {
    use {super::*, itoa::Buffer};

    pub fn serialize<S>(id: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Id::from(*id).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let id: Id = Deserialize::deserialize(deserializer)?;

        Ok(id.into())
    }

    #[derive(Debug, Clone, Copy)]
    struct Id {
        id: u64,
    }

    impl Id {
        fn new(id: u64) -> Self {
            Self { id }
        }
    }

    impl<'de> Deserialize<'de> for Id {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            struct IdVisitor;

            impl<'de> de::Visitor<'de> for IdVisitor {
                type Value = Id;

                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    f.write_str("a JIRA object id")
                }

                fn visit_u64<E>(self, id: u64) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    Ok(Id::new(id))
                }

                fn visit_str<E>(self, id: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    let id: u64 = id.parse().map_err(de::Error::custom)?;

                    Ok(Id::new(id))
                }
            }

            deserializer.deserialize_any(IdVisitor)
        }
    }

    impl Serialize for Id {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(Buffer::new().format(self.id))
        }
    }

    impl From<u64> for Id {
        fn from(n: u64) -> Self {
            Self::new(n)
        }
    }

    impl From<u32> for Id {
        fn from(n: u32) -> Self {
            Self::new(n as u64)
        }
    }

    impl From<u16> for Id {
        fn from(n: u16) -> Self {
            Self::new(n as u64)
        }
    }

    impl From<u8> for Id {
        fn from(n: u8) -> Self {
            Self::new(n as u64)
        }
    }

    impl From<Id> for u64 {
        fn from(id: Id) -> Self {
            id.id
        }
    }
}

#[cfg(test)]
pub(crate) mod types {
    use serde_json::{json, Value as Json};

    pub fn user() -> Json {
        json!({
            "active": true,
            "avatarUrls": {
                "foo": "bar",
            },
            "displayName": "foo",
            "emailAddress": "foo",
            "key": "foo",
            "name": "foo",
            "self": "foo",
            "timeZone": "foo",
        })
    }

    pub fn status() -> Json {
        json!({
            "description": "foo",
            "iconUrl": "foo",
            "id": "42",
            "name": "foo",
            "self": "foo",
        })
    }

    pub fn issuetype() -> Json {
        json!({
            "description": "foo",
            "iconUrl": "foo",
            "id": "42",
            "name": "foo",
            "self": "foo",
            "subtask": true,
        })
    }

    pub fn version() -> Json {
        json!({
            "archived": true,
            "id": "42",
            "name": "foo",
            "released": true,
            "self": "foo",
        })
    }

    pub fn comment() -> Json {
        json!({
            "id": "42",
            "self": "foo",
            "author": null,
            "update_author": null,
            "created": "foo",
            "updated": "foo",
            "body": "foo",
            "visibility": null,
        })
    }

    pub fn comments() -> Json {
        json!([comment(), comment(),])
    }

    pub fn visibility() -> Json {
        json!({
            "type": "foo",
            "value": "foo",
        })
    }

    pub fn project() -> Json {
        json!({
            "id": "42",
            "self": "foo",
            "key": "foo",
            "name": "foo",
        })
    }

    pub fn issuelink() -> Json {
        json!({
            "id": "42",
            "self": "foo",
            "outwardIssue": null,
            "inwardIssue": null,
            "type": linktype(),
        })
    }

    pub fn linktype() -> Json {
        json!({
                "id": "42",
                "self": "foo",
                "inward": "foo",
                "outward": "foo",
                "name": "foo",
        })
    }

    pub fn resolution() -> Json {
        json!({
            "name": "foo"
        })
    }

    pub fn attachment() -> Json {
        json!({
            "id": "42",
            "self": "foo",
            "filename": "foo",
            "author": user(),
            "created": "foo",
            "size": 42,
            "mimeType": "foo",
            "content": "foo",
            "thumbnail": "foo",
        })
    }

    pub fn priority() -> Json {
        json!({
            "id": "42",
            "self": "foo",
            "iconUrl": "foo",
            "name": "foo",
        })
    }

    pub fn nested_response() -> Json {
        json!({
            "status": 200,
            "errorCollection": error_collection(),
        })
    }

    pub fn error_collection() -> Json {
        json!({
            "errorMessages": ["foo", "bar"],
            "errors": {"foo": "bar"},
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    macro_rules! jbytes {
        ($($json:tt)+) => {
            serde_json::to_vec(&json!($($json)+)).expect("Failed to serialize json in test macro... this is a bug")
        }
    }

    #[test]
    fn deserialize_user() {
        let json = jbytes!(types::user());
        let user: Result<User, _> = deserialize(&json);

        assert!(user.is_ok())
    }

    #[test]
    fn deserialize_status() {
        let json = jbytes!(types::status());

        let status: Result<Status, _> = deserialize(&json);

        assert!(status.is_ok())
    }

    #[test]
    fn deserialize_issuetype() {
        let json = jbytes!(types::issuetype());

        let issuetype: Result<IssueType, _> = deserialize(&json);

        assert!(issuetype.is_ok())
    }

    #[test]
    fn deserialize_version() {
        let json = jbytes!(types::version());

        let version: Result<Version, _> = deserialize(&json);

        assert!(version.is_ok())
    }

    #[test]
    fn deserialize_comment() {
        let json = jbytes!(types::comment());

        let comment: Result<Comment, _> = deserialize(&json);

        assert!(comment.is_ok())
    }

    #[test]
    fn deserialize_comments() {
        let json = jbytes!(types::comments());

        let comments: Result<Comments, _> = deserialize(&json);

        assert!(comments.is_ok())
    }

    #[test]
    fn deserialize_visibility() {
        let json = jbytes!(types::visibility());

        let viz: Result<Visibility, _> = deserialize(&json);

        assert!(viz.is_ok())
    }

    #[test]
    fn deserialize_project() {
        let json = jbytes!(types::project());

        let project: Result<Project, _> = deserialize(&json);

        assert!(project.is_ok())
    }

    #[test]
    fn deserialize_issuelink() {
        let json = jbytes!(types::issuelink());

        let issuelink: Result<IssueLink, _> = deserialize(&json);

        assert!(issuelink.is_ok())
    }

    #[test]
    fn deserialize_linktype() {
        let json = jbytes!(types::linktype());

        let issuetype: Result<LinkType, _> = deserialize(&json);

        assert!(issuetype.is_ok())
    }

    #[test]
    fn deserialize_resolution() {
        let json = jbytes!(types::resolution());

        let resolution: Result<Resolution, _> = deserialize(&json);

        assert!(resolution.is_ok())
    }

    #[test]
    fn deserialize_attachment() {
        let json = jbytes!(types::attachment());

        let attachment: Result<Attachment, _> = deserialize(&json);

        assert!(attachment.is_ok())
    }

    #[test]
    fn deserialize_priority() {
        let json = jbytes!(types::priority());

        let priority: Result<Priority, _> = deserialize(&json);

        assert!(priority.is_ok())
    }

    #[test]
    fn deserialize_nested_response() {
        let json = jbytes!(types::nested_response());

        let response: Result<NestedResponse, _> = deserialize(&json);

        assert!(response.is_ok())
    }

    #[test]
    fn deserialize_error_collection() {
        let json = jbytes!(types::error_collection());

        let collection: Result<ErrorCollection, _> = deserialize(&json);

        assert!(collection.is_ok())
    }

    fn deserialize<'de, 'a: 'de, T>(bytes: &'a [u8]) -> Result<T, serde_json::Error>
    where
        T: Deserialize<'de>,
    {
        let value = serde_json::from_slice(bytes).map_err(|error| {
            dbg!(&error);
            error
        });

        value
    }
}
