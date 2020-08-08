use {
    super::*,
    json::{value::RawValue as RawJson, Error as JsonError},
    serde::{Deserialize, Serialize, Serializer},
    serde_json as json,
};

/// Interface for accessing a zero copy representation
/// of a call to JIRA's issue creation metadata API endpoint.
#[derive(Debug, Deserialize)]
#[serde(try_from = "Box<RawJson>")]
pub struct MetaCreate {
    // This handle must never be exposed in the public API
    inner: handle::CreateMetaHandle,
}

impl MetaCreate {
    /// Try instantiate a new handle with the given backing JSON
    pub fn try_new(store: Box<RawJson>) -> Result<Self, JsonError> {
        let inner =
            handle::CreateMetaHandle::try_new_or_drop(store, |json| json::from_str(json.get()))?;

        Ok(Self { inner })
    }

    /// Access this handle's data
    pub fn data(&self) -> &IssueMetadata {
        self.inner.suffix()
    }

    /// Consume the handle returning the backing
    /// storage
    pub fn into_inner(self) -> Box<RawJson> {
        self.inner.into_head()
    }
}

impl TryFrom<Box<RawJson>> for MetaCreate {
    type Error = JsonError;

    fn try_from(value: Box<RawJson>) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

// Delegate the serializer to the internal handle
impl Serialize for MetaCreate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data().serialize(serializer)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueMetadata<'a> {
    pub expand: Option<&'a str>,
    #[serde(borrow)]
    pub projects: Vec<IssueTypeMeta<'a>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueTypeMeta<'a> {
    #[serde(flatten, borrow)]
    pub issue_type: IssueType<'a>,
    // Only exists when API is queried with 'expand=projects.issues.fields'
    pub fields: Option<Vec<IssueFieldsMeta<'a>>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueFieldsMeta<'a> {
    pub required: bool,
    pub name: &'a str,
    #[serde(rename = "fieldId")]
    pub field_id: &'a str,
    #[serde(rename = "defaultValue")]
    pub default: Option<&'a RawJson>,
    pub schema: Option<FieldSchema<'a>>,
    pub operations: Vec<Operations>,
    #[serde(rename = "allowedValues")]
    pub possible_values: Option<Vec<&'a RawJson>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldSchema<'a> {
    // TODO: Try to find an authoritative source on what types it can be
    /// One of: any,array,date,issuetype,number,option,
    /// priority,project,string,timestracking,user
    /// Probably has more variants
    #[serde(rename = "field")]
    pub field_type: &'a str,
    // Mutually exclusive with 'custom'
    pub system: Option<&'a str>,
    pub custom: Option<&'a str>,
    // Only exists if 'custom' exists
    #[serde(rename = "fieldId")]
    pub custom_id: Option<u64>,
    // Only exists if 'field_type' == array
    pub items: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Operations {
    Set,
    Add,
    Remove,
}

rental! {
    mod handle {
        use super::*;

        #[rental(debug, covariant)]
        pub(super) struct CreateMetaHandle {
            store: Box<RawJson>,
            handle: IssueMetadata<'store>,
        }
    }
}
