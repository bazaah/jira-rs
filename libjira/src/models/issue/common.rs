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
    #[serde(with = "id")]
    pub id: u64,
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
    #[serde(with = "id")]
    pub id: u64,
    pub name: &'a str,
    #[serde(rename = "self")]
    pub self_link: &'a str,
    pub subtask: bool,
}

// TODO: Investigate where this shows up
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Version<'a> {
    pub archived: bool,
    #[serde(with = "id")]
    pub id: u64,
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
    #[serde(with = "id")]
    pub id: u64,
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
    #[serde(with = "id")]
    pub id: u64,
    #[serde(rename = "self")]
    pub self_link: &'a str,
    pub key: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueLink<'a> {
    #[serde(with = "id")]
    pub id: u64,
    #[serde(rename = "self")]
    pub self_link: &'a str,
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
    #[serde(with = "id")]
    pub id: u64,
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
    #[serde(with = "id")]
    pub id: u64,
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
    D: de::Deserializer<'de>,
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
