use {
    jira_rs::client::Authentication,
    std::str::FromStr,
    structopt::{clap::ArgSettings, StructOpt},
};

#[derive(Debug, StructOpt)]
#[structopt(name = "jira", rename_all = "kebab")]
pub struct CliOptions {
    /// Invoke requests against this host
    #[structopt(env = "JIRA_HOST", short = "H", long)]
    host: String,
    /// Use these authentication credentials for command invocations
    #[structopt(env = "JIRA_AUTH", short = "A", long, hide_env_values = true)]
    auth: String,
    /// Increase log verbosity
    ///
    /// Repeat the command for higher log levels
    #[structopt(short = "v", long = "verbosity", parse(from_occurrences = get_level))]
    debug_level: u8,

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(bin_name = "jira issues")]
    Issues(Issues),
}

/// Interact with Jira issues
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab")]
pub enum Issues {
    /// Get a single issue by key or id
    Get {
        /// The issue key or id to retrieve
        #[structopt(value_name = "KEY/ID")]
        key: String,

        #[structopt(flatten)]
        opts: options::IssuesGet,
    },
    /// Search for issues using a JQL query
    Search {
        /// JQL query string to search with
        ///
        /// For more information on the Jira Query Language (JQL) visit
        /// https://support.atlassian.com/jira-software-cloud/docs/what-is-advanced-searching-in-jira-cloud
        #[structopt(value_name = "JQL")]
        jql: String,

        #[structopt(flatten)]
        opts: options::IssuesSearch,
    },
    /// Find metadata for creating or editing issues
    Meta {
        #[structopt(flatten)]
        opts: options::IssueMetadata,
    },
}

impl CliOptions {
    pub fn new() -> Self {
        Self::from_args()
    }

    pub(crate) fn auth_user(&self) -> Option<&str> {
        self.auth.split(':').nth(0)
    }

    pub(crate) fn auth_password(&self) -> Option<&str> {
        self.auth.split(':').nth(1)
    }

    pub(crate) fn authentication(&self) -> Option<Authentication> {
        self.auth_user().and_then(|user| {
            self.auth_password()
                .map(|pw| Authentication::basic(user, pw))
        })
    }

    pub(crate) fn host(&self) -> &str {
        self.host.as_str()
    }
}

fn get_level(o: u64) -> u8 {
    match o {
        n @ 0..=4 => n as u8,
        _ => 5,
    }
}

pub mod options {
    use super::*;
    use jira_rs::issue::options;

    #[derive(Debug, StructOpt)]
    #[structopt(rename_all = "kebab")]
    pub struct IssuesGet {
        /// Should the --fields be treated as keys not ids?
        #[structopt(short = "k", long = "by-key")]
        pub fields_by_key: bool,

        /// Should this request update the user's search history?
        ///
        /// Specifically, the user's last.Viewed issue and the
        /// user's recently viewed Projects
        #[structopt(short, long)]
        pub update_history: bool,

        /// List of fields from the issue to return
        ///
        /// By default, all fields are returned.
        ///
        /// Special
        /// ['*navigable'] will return navigable fields
        /// '*all' will return all fields
        ///
        /// Modifiers
        /// '-' A dash prefixed to any non special field will omit the field
        #[structopt(short, long, value_name = ",delimited", set = ArgSettings::AllowLeadingHyphen)]
        pub fields: Option<String>,

        /// List of expands to return in the response
        ///
        /// Possible values
        /// 'renderedFields', 'names', 'schema', 'transitions', 'editmeta', 'changelog', 'versionedRepresentations'
        #[structopt(short, long, value_name = ",delimited")]
        pub expand: Option<String>,

        /// List of properties to return
        ///
        /// Special
        /// '*all' will return all properties
        ///
        /// Modifiers
        /// '-' A dash prefixed to any non special property will omit the property
        #[structopt(short, long, value_name = ",delimited", set = ArgSettings::AllowLeadingHyphen)]
        pub properties: Option<String>,
    }

    impl<'a> Into<options::Get> for &'a IssuesGet {
        fn into(self) -> options::Get {
            options::Get::new()
                .with_fields(self.fields.as_ref().map(|s| s.split(",")))
                .expand(self.expand.as_ref().map(|s| s.split(",")))
                .fields_by_key(Some(self.fields_by_key))
                .properties(self.properties.as_ref().map(|s| s.split(",")))
                .update_history(Some(self.update_history))
        }
    }

    #[derive(Debug, StructOpt)]
    #[structopt(rename_all = "kebab")]
    pub struct IssuesSearch {
        /// Should the --fields be treated as keys not ids?
        #[structopt(short = "k", long = "by-key")]
        pub fields_by_key: bool,

        /// List of expands to return in the response
        ///
        /// Possible values
        /// 'renderedFields', 'names', 'schema', 'transitions', 'editmeta', 'changelog', 'versionedRepresentations'
        #[structopt(short, long, value_name = ",delimited")]
        pub expand: Option<String>,

        /// List of fields from the issue to return
        ///
        /// By default, all fields are returned.
        ///
        /// Special
        /// ['*navigable'] will return navigable fields
        /// '*all' will return all fields
        ///
        /// Modifiers
        /// '-' A dash prefixed to any non special field will omit the field
        #[structopt(short, long, value_name = ",delimited", set = ArgSettings::AllowLeadingHyphen)]
        pub fields: Option<String>,

        /// Maximum number of issues to return
        #[structopt(short, long, value_name = "uint")]
        pub max_results: Option<u32>,

        /// List of properties to return in each issue
        ///
        /// Special
        /// '*all' - all properties
        ///
        /// Modifiers
        /// '-' A dash prefixed to any non special property will omit the property
        #[structopt(short, long, value_name = ",delimited", set = ArgSettings::AllowLeadingHyphen)]
        pub properties: Option<String>,

        /// Return results starting from
        ///
        /// This option is not affected by --max_results.
        /// When using both, the returned range is: start_at..max_results
        #[structopt(short, long, value_name = "uint")]
        pub start_at: Option<u32>,

        /// Validation mode of the JQL query
        ///
        /// Possible values
        /// ['strict'], 'warn', 'none'
        #[structopt(short, long, value_name = "mode", parse(try_from_str = try_into_validate))]
        pub validate: Option<options::ValidateQuery>,
    }

    impl<'a> Into<options::Search> for &'a IssuesSearch {
        fn into(self) -> options::Search {
            options::Search::new()
                .start_at(self.start_at)
                .max_results(self.max_results)
                .validate(self.validate)
                .with_fields(self.fields.as_ref().map(|s| s.split(",")))
                .expand(self.expand.as_ref().map(|s| s.split(",")))
                .fields_by_key(Some(self.fields_by_key))
                .properties(self.properties.as_ref().map(|s| s.split(",")))
        }
    }

    #[derive(Debug, StructOpt)]
    #[structopt(rename_all = "kebab")]
    pub struct IssueMetadata {
        /// List of projects to return issue-types for
        ///
        /// Default to returning all available projects.
        #[structopt(short = "P", long, value_name = "KEY/ID")]
        pub projects: Option<Vec<String>>,

        /// List of issue-types to return schemas for
        ///
        /// Default to returning all available issue types.
        #[structopt(short = "I", long, value_name = "KEY/ID")]
        pub issue_types: Option<Vec<String>>,

        /// Don't return schemas, only project/issue-type layout
        ///
        /// Useful for exploring possible values for --projects and --issue-types
        #[structopt(short, long)]
        pub short: bool,
    }

    impl<'a> Into<options::MetaCreate> for &'a IssueMetadata {
        fn into(self) -> options::MetaCreate {
            use options::MetaCreate;

            let opts = self
                .projects
                .as_ref()
                .map(|v| {
                    v.into_iter()
                        .fold(MetaCreate::new(), |o, ref i| match u64::from_str(i) {
                            Ok(int) => o.project_ids(Some(Some(int).into_iter())),
                            Err(_) => o.project_keys(Some(Some(i.as_str()).into_iter())),
                        })
                })
                .and_then(|options| {
                    self.issue_types.as_ref().map(|v| {
                        v.into_iter()
                            .fold(options, |o, ref i| match u64::from_str(i) {
                                Ok(int) => o.issuetype_ids(Some(Some(int).into_iter())),
                                Err(_) => o.issuetype_keys(Some(Some(i.as_str()).into_iter())),
                            })
                    })
                })
                .unwrap_or_default();

            match self.short {
                true => opts,
                false => opts.expand(Some(Some("projects.issuetypes.fields").into_iter())),
            }
        }
    }

    fn try_into_validate(input: &str) -> Result<options::ValidateQuery, String> {
        options::ValidateQuery::try_new(input).ok_or_else(|| {
            format!(
                "expected one of [{}], got '{}'",
                "strict, warn, none", input
            )
        })
    }
}
