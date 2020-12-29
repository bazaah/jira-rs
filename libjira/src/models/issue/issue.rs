use jsonp::{Pointer, Segment};

use {
    super::*,
    json::{value::RawValue as RawJson, Error as JsonError},
    serde::{Deserializer, Serializer},
    serde_json as json,
    std::collections::HashMap,
};

#[derive(Debug, Deserialize)]
#[serde(try_from = "Box<RawJson>")]
pub struct IssueHandle {
    // This handle must never be exposed in the public API
    inner: handle::IssueInner,
}

impl IssueHandle {
    /// Try instantiate a new handle with the given backing JSON
    pub fn try_new(store: Box<RawJson>) -> Result<Self, JsonError> {
        let inner = handle::IssueInner::try_new(store, |raw| json::from_str(raw.get()))?;

        Ok(Self { inner })
    }

    /// Access this handle's data
    pub fn data(&self) -> &Issue {
        self.inner.borrow_handle()
    }

    /// Consume the handle returning the backing
    /// storage
    pub fn into_inner(self) -> Box<RawJson> {
        self.inner.into_heads().store
    }
}

impl TryFrom<Box<RawJson>> for IssueHandle {
    type Error = JsonError;

    fn try_from(value: Box<RawJson>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

// Delegate the serializer to the internal handle
impl Serialize for IssueHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data().serialize(serializer)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Issue<'a> {
    #[serde(rename = "self")]
    pub self_link: &'a str,
    #[serde(with = "common::id")]
    pub id: u64,
    pub key: &'a str,
    pub expand: &'a str,
    pub fields: HashMap<&'a str, &'a RawJson>,

    // Capture any extra fields returned (if any)
    #[serde(flatten)]
    pub extra: HashMap<&'a str, &'a RawJson>,
}

impl<'a> Issue<'a> {
    const FIELDS: &'static str = "fields";

    /// Attempt to deserialize an arbitrary value from the `fields` with the given dot `.`
    /// delimited json pointer.
    ///
    /// Examples
    ///
    /// // Access the first inward issue's id
    /// issue.field("issuelinks.0.inwardIssue.id")
    ///
    /// // Access a custom field
    /// issue.field("customfield_10000")
    pub fn field<'de, T>(&self, dotted: &str) -> Option<Result<T, JsonError>>
    where
        T: Deserialize<'de>,
        'a: 'de,
    {
        self.field_with(dotted.split("."))
    }

    /// Attempt to deserialize an arbitrary value from the `fields` with the given pointer
    /// segments.
    pub fn field_with<'de, 'i, T, I>(&self, ptr: I) -> Option<Result<T, JsonError>>
    where
        I: Iterator<Item = &'i str>,
        T: Deserialize<'de>,
        'a: 'de,
    {
        let ptr = Some(Self::FIELDS).into_iter().chain(ptr);

        self.access_with(ptr)
    }

    /// Attempt to access and deserialize an arbitrary value with the given dot `.`
    /// delimited pointer.
    ///
    /// This can be used to access any objects in the `.fields` map, or any
    /// nonstandard `.extra`s that specific Jira instances may add.
    ///
    /// Examples
    ///
    /// // Access a known object under the `fields` map
    /// issue.access("fields.creator.id")
    ///
    /// // Access a nonstandard instance specific field
    /// issue.access("nonstandard.jira.key")
    pub fn access<'de, T>(&self, dotted: &str) -> Option<Result<T, JsonError>>
    where
        T: Deserialize<'de>,
        'a: 'de,
    {
        self.access_with(dotted.split("."))
    }

    /// Attempt to access and deserialize an arbitrary value with the given pointer
    /// segments.
    pub fn access_with<'de, 'i, T, I>(&self, ptr: I) -> Option<Result<T, JsonError>>
    where
        I: Iterator<Item = &'i str>,
        T: Deserialize<'de>,
        'a: 'de,
    {
        let mut ptr = ptr;
        let key = ptr.next()?;

        let map = match key {
            Self::FIELDS => self.fields.get(ptr.next()?),
            _ => self.extra.get(key),
        };

        map.map(|&raw| Pointer::default().with_segments(raw, ptr.map(Segment::lazy)))
    }

    fn string_field(&self, key: &str) -> Option<Result<&str, JsonError>> {
        self.field(key)
    }

    fn user_field(&self, key: &str) -> Option<Result<User, JsonError>> {
        self.field(key)
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
        self.field("status").and_then(Result::ok)
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
        self.field("issuetype").and_then(Result::ok)
    }

    /// Labels assigned to this issue
    pub fn labels(&self) -> Option<Vec<&str>> {
        self.field("labels").and_then(Result::ok)
    }

    /// Issue fix version(s)
    pub fn fix_versions(&self) -> Option<Vec<Version>> {
        self.field("fixVersions").and_then(Result::ok)
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
        self.field("priority").and_then(Result::ok)
    }

    /// Other Issues that are linked to the current Issue
    pub fn issue_links(&self) -> Option<Vec<IssueLink>> {
        self.field("issuelinks").and_then(Result::ok)
    }

    /// The project this Issue is assigned to
    pub fn project(&self) -> Option<Project> {
        self.field("project").and_then(Result::ok)
    }

    /// This Issue's resolution, if it exists
    pub fn resolution(&self) -> Option<Resolution> {
        self.field("resolution").and_then(Result::ok)
    }

    /// Any attachments this Issue contains
    pub fn attachment(&self) -> Option<Vec<Attachment>> {
        self.field("attachment").and_then(Result::ok)
    }
}

/*
 * Note that we write out our deserialize impl for Issue because
 * `serde_json::RawValue` is currently bugged and will automatically fail
 * deserialization when used in a structure with `serde(flatten)` on it,
 * as we do here with `extra`.
 *
 * TODO: Revert to macro deserialize when this is fixed
 * Tracking issue: https://github.com/serde-rs/json/issues/599
 */
impl<'a, 'de: 'a> Deserialize<'de> for Issue<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::{fmt, marker::PhantomData};

        /// Wrapper for forwarding the deserialization impl to
        /// `common::id`'s impl
        struct IdDeserializer {
            value: u64,
        }

        impl<'de> Deserialize<'de> for IdDeserializer {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(IdDeserializer {
                    value: common::id::deserialize(deserializer)?,
                })
            }
        }

        /// Custom Visitor for deserializing Issue
        struct IssueVisitor<'a>(PhantomData<fn() -> Issue<'a>>);
        impl<'a> IssueVisitor<'a> {
            const SELF_LINK: &'static str = "self";
            const ID: &'static str = "id";
            const KEY: &'static str = "key";
            const EXPAND: &'static str = "expand";
            const FIELDS: &'static str = "fields";
            const EXTRA: &'static str = "extra";
        }

        impl<'a, 'de: 'a> Visitor<'de> for IssueVisitor<'a> {
            type Value = Issue<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Jira issue")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let dupe = |field| de::Error::duplicate_field(field);
                let missing = |field| de::Error::missing_field(field);

                let mut self_link = None;
                let mut id: Option<IdDeserializer> = None;
                let mut key = None;
                let mut expand = None;
                let mut fields = None;
                let mut extra: Option<HashMap<&str, &RawJson>> = None;

                while let Some(map_key) = access.next_key()? {
                    match map_key {
                        Self::SELF_LINK if self_link.is_some() => {
                            return Err(dupe(Self::SELF_LINK))
                        }
                        Self::SELF_LINK => self_link = Some(access.next_value()?),

                        Self::ID if id.is_some() => return Err(dupe(Self::ID)),
                        Self::ID => id = Some(access.next_value()?),

                        Self::KEY if key.is_some() => return Err(dupe(Self::KEY)),
                        Self::KEY => key = Some(access.next_value()?),

                        Self::EXPAND if expand.is_some() => return Err(dupe(Self::EXPAND)),
                        Self::EXPAND => expand = Some(access.next_value()?),

                        Self::FIELDS if fields.is_some() => return Err(dupe(Self::FIELDS)),
                        Self::FIELDS => fields = Some(access.next_value()?),

                        unknown => match extra {
                            Some(ref mut map) => {
                                map.insert(unknown, access.next_value()?);
                            }
                            None => {
                                let mut map = HashMap::new();
                                map.insert(unknown, access.next_value()?);

                                extra = Some(map)
                            }
                        },
                    }
                }

                Ok(Issue {
                    self_link: self_link.ok_or_else(|| missing(Self::SELF_LINK))?,
                    id: id.ok_or_else(|| missing(Self::ID))?.value,
                    key: key.ok_or_else(|| missing(Self::KEY))?,
                    expand: expand.ok_or_else(|| missing(Self::EXPAND))?,
                    fields: fields.ok_or_else(|| missing(Self::FIELDS))?,
                    extra: extra.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_map(IssueVisitor(Default::default()))
    }
}

mod handle {
    use super::*;
    use ouroboros::self_referencing as ouroboros;

    #[ouroboros(pub_extras)]
    #[derive(Debug)]
    pub(super) struct IssueInner {
        store: Box<RawJson>,
        #[borrows(store)]
        pub(super) handle: Issue<'this>,
    }
}

#[cfg(test)]
pub(crate) mod types {
    use crate::models::issue::common;
    use serde_json::{json, Value as Json};

    pub fn issue() -> Json {
        json!({
            "self": "foo",
            "id": "42",
            "key": "foo",
            "expand": "foo",
            "fields": {
                "assignee": common::types::user(),
                "creator": common::types::user(),
                "reporter": common::types::user(),
                "summary": "foo",
                "status": common::types::status(),
                "description": "foo",
                "updated": "foo",
                "created": "foo",
                "resolutiondate": "foo",
                "issuetype": common::types::issuetype(),
                "labels": ["foo", "bar"],
                "fixVersions": [common::types::version()],
                "comment": {
                    "comments": common::types::comments()
                },
                "issuelinks": [common::types::issuelink()],
                "priority": common::types::priority(),
                "resolution": common::types::resolution(),
                "attachment": common::types::attachment(),
            },
            "nonstandard": "field",
            "another": "strange field",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as Json;

    #[test]
    fn deserialize_issue_handle() {
        let json = jbytes(types::issue());

        let handle: Result<IssueHandle, _> = deserialize(&json);

        assert!(handle.is_ok())
    }

    #[test]
    fn deserialize_issue() {
        let json = jbytes(types::issue());

        println!("{}", serde_json::to_string_pretty(&types::issue()).unwrap());

        let issue: Result<Issue, _> = deserialize(&json);

        assert!(issue.is_ok())
    }

    fn jbytes(json: Json) -> Vec<u8> {
        serde_json::to_vec(&json)
            .expect("Failed to serialize in models/issue/issue tests... this is a bug")
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
