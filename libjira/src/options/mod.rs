use {
    reqwest::RequestBuilder,
    serde::{Serialize, Serializer},
    std::fmt,
};

pub mod issue;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum ValidateQuery {
    Strict,
    Warn,
    None,
}

pub(crate) trait ApiOptions {
    fn valid_options(&self) -> &[OptRef];

    fn jql(&self) -> Option<&str> {
        None
    }

    fn start_at(&self) -> Option<u32> {
        None
    }

    fn max_results(&self) -> Option<u32> {
        None
    }

    fn validate_query(&self) -> Option<ValidateQuery> {
        None
    }

    fn with_fields(&self) -> Option<&[String]> {
        None
    }

    fn expand(&self) -> Option<&[String]> {
        None
    }

    fn properties(&self) -> Option<&[String]> {
        None
    }

    fn fields_by_key(&self) -> Option<bool> {
        None
    }

    fn update_history(&self) -> Option<bool> {
        None
    }
}

pub(crate) fn with_options<O>(request: RequestBuilder, options: Option<&O>) -> RequestBuilder
where
    O: ApiOptions,
{
    match options {
        Some(options) => {
            let valid = options.valid_options();
            valid.iter().fold(request, |req: RequestBuilder, &opt| {
                apply_option(req, opt, options)
            })
        }
        None => request,
    }
}

fn apply_option<O>(request: RequestBuilder, visit: OptRef, options: &O) -> RequestBuilder
where
    O: ApiOptions,
{
    match visit {
        OptRef::Jql => add_query(
            request,
            options.jql().map(|o| [(queryKey::JQL, o)]).as_ref(),
        ),
        OptRef::StartAt => add_query(
            request,
            options
                .start_at()
                .map(|o| [(queryKey::START_AT, o)])
                .as_ref(),
        ),
        OptRef::MaxResults => add_query(
            request,
            options
                .max_results()
                .map(|o| [(queryKey::MAX_RESULTS, o)])
                .as_ref(),
        ),
        OptRef::ValidateQuery => add_query(
            request,
            options
                .validate_query()
                .map(|o| [(queryKey::VALIDATE_QUERY, o)])
                .as_ref(),
        ),
        OptRef::WithFields => add_query(
            request,
            options
                .with_fields()
                .map(|o| [(queryKey::WITH_FIELDS, CommaDelimited::new(o))])
                .as_ref(),
        ),
        OptRef::Expand => add_query(
            request,
            options
                .expand()
                .map(|o| [(queryKey::EXPAND, CommaDelimited::new(o))])
                .as_ref(),
        ),
        OptRef::Properties => add_query(
            request,
            options
                .properties()
                .map(|o| [(queryKey::PROPERTIES, CommaDelimited::new(o))])
                .as_ref(),
        ),
        OptRef::FieldsByKey => add_query(
            request,
            options
                .fields_by_key()
                .map(|o| [(queryKey::FIELDS_BY_KEY, o)])
                .as_ref(),
        ),
        OptRef::UpdateHistory => add_query(
            request,
            options
                .update_history()
                .map(|o| [(queryKey::UPDATE_HISTORY, o)])
                .as_ref(),
        ),
    }
}

#[inline]
fn add_query<S>(request: RequestBuilder, query: Option<&S>) -> RequestBuilder
where
    S: Serialize + ?Sized,
{
    if query.is_some() {
        request.query(query.unwrap())
    } else {
        request
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum OptRef {
    Jql,
    StartAt,
    MaxResults,
    ValidateQuery,
    WithFields,
    Expand,
    Properties,
    FieldsByKey,
    UpdateHistory,
}

#[derive(Debug)]
struct CommaDelimited<'a> {
    inner: &'a [String],
}

impl<'a> CommaDelimited<'a> {
    fn new(inner: &'a [String]) -> Self {
        Self { inner }
    }
}

impl<'a> fmt::Display for CommaDelimited<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.inner.len();

        self.inner
            .iter()
            .map(|s| s.as_str())
            .enumerate()
            .try_for_each(|(i, element)| {
                if i < len {
                    write!(f, "{},", element)
                } else {
                    write!(f, "{}", element)
                }
            })
    }
}

impl<'a> Serialize for CommaDelimited<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

#[allow(non_snake_case)]
mod queryKey {
    pub(super) const JQL: &'static str = "jql";
    pub(super) const START_AT: &'static str = "startAt";
    pub(super) const MAX_RESULTS: &'static str = "maxResults";
    pub(super) const VALIDATE_QUERY: &'static str = "validateQuery";
    pub(super) const WITH_FIELDS: &'static str = "fields";
    pub(super) const EXPAND: &'static str = "expand";
    pub(super) const PROPERTIES: &'static str = "properties";
    pub(super) const FIELDS_BY_KEY: &'static str = "fieldsByKeys";
    pub(super) const UPDATE_HISTORY: &'static str = "updateHistory";
}
