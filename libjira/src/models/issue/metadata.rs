use {
    super::*,
    json::{value::RawValue as RawJson, Error as JsonError},
    serde::Serializer,
    serde_json as json,
    std::collections::HashMap,
};

/// Interface for accessing a zero copy representation
/// of a call to JIRA's issue creation metadata API endpoint.
#[derive(Debug, Deserialize)]
#[serde(try_from = "Box<RawJson>")]
pub struct MetaCreateHandle {
    // This handle must never be exposed in the public API
    inner: handle::MetaCreateInner,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetaCreate<'a> {
    pub expand: Option<&'a str>,
    #[serde(borrow)]
    pub projects: Vec<ProjectMeta<'a>>,
}

impl MetaCreateHandle {
    /// Try instantiate a new handle with the given backing JSON
    pub fn try_new(store: Box<RawJson>) -> Result<Self, JsonError> {
        let inner = handle::MetaCreateInner::try_new(store, |raw| json::from_str(raw.get()))?;

        Ok(Self { inner })
    }

    /// Access this handle's data
    pub fn data(&self) -> &MetaCreate {
        self.inner.borrow_handle()
    }

    /// Consume the handle returning the backing
    /// storage
    pub fn into_inner(self) -> Box<RawJson> {
        self.inner.into_heads().store
    }
}

impl TryFrom<Box<RawJson>> for MetaCreateHandle {
    type Error = JsonError;

    fn try_from(value: Box<RawJson>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

// Delegate the serializer to the internal handle
impl Serialize for MetaCreateHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data().serialize(serializer)
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "Box<RawJson>")]
pub struct MetaEditHandle {
    // This handle must never be exposed in the public API
    inner: handle::MetaEditInner,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetaEdit<'a> {
    #[serde(borrow)]
    pub fields: HashMap<&'a str, IssueFieldsMeta<'a>>,
}

impl MetaEditHandle {
    /// Try instantiate a new handle with the given backing JSON
    pub fn try_new(store: Box<RawJson>) -> Result<Self, JsonError> {
        let inner = handle::MetaEditInner::try_new(store, |raw| json::from_str(raw.get()))?;

        Ok(Self { inner })
    }

    /// Access this handle's data
    pub fn data(&self) -> &MetaEdit {
        self.inner.borrow_handle()
    }

    /// Consume the handle returning the backing
    /// storage
    pub fn into_inner(self) -> Box<RawJson> {
        self.inner.into_heads().store
    }
}

impl TryFrom<Box<RawJson>> for MetaEditHandle {
    type Error = JsonError;

    fn try_from(value: Box<RawJson>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

// Delegate the serializer to the internal handle
impl Serialize for MetaEditHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data().serialize(serializer)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectMeta<'a> {
    #[serde(flatten, borrow)]
    pub project: Project<'a>,
    #[serde(rename = "issuetypes", borrow)]
    issue_types: Vec<IssueTypeMeta<'a>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueTypeMeta<'a> {
    #[serde(flatten, borrow)]
    pub issue_type: IssueType<'a>,
    // Only exists when API is queried with 'expand=projects.issues.fields'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<&'a str, IssueFieldsMeta<'a>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueFieldsMeta<'a> {
    pub required: bool,
    pub name: &'a str,
    #[serde(rename = "fieldId")]
    pub field_id: &'a str,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default: Option<&'a RawJson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<FieldSchema<'a>>,
    pub operations: Vec<Operations>,
    #[serde(rename = "allowedValues", skip_serializing_if = "Option::is_none")]
    pub possible_values: Option<Vec<&'a RawJson>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldSchema<'a> {
    // TODO: Try to find an authoritative source on what types it can be
    /// One of: any,array,date,issuetype,number,option,
    /// priority,project,string,timestracking,user
    /// Probably has more variants
    #[serde(rename = "type")]
    pub field_type: &'a str,
    // Mutually exclusive with 'custom'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<&'a str>,
    // Only exists if 'custom' exists
    #[serde(rename = "customId", skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<u64>,
    // Only exists if 'field_type' == array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Operations {
    Set,
    Edit,
    Add,
    Remove,
}

mod handle {
    use super::*;
    use ouroboros::self_referencing as ouroboros;

    #[ouroboros(pub_extras)]
    #[derive(Debug)]
    pub(super) struct MetaCreateInner {
        store: Box<RawJson>,
        #[borrows(store)]
        pub(super) handle: MetaCreate<'this>,
    }

    #[ouroboros(pub_extras)]
    #[derive(Debug)]
    pub(super) struct MetaEditInner {
        store: Box<RawJson>,
        #[borrows(store)]
        pub(super) handle: MetaEdit<'this>,
    }
}

