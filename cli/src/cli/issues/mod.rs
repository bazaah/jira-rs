use {super::*, jira_rs::issue::options as IssueOptions};

pub mod create;
pub mod get;
pub mod meta;
pub mod search;

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
        opts: get::IssuesGet,
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
        opts: search::IssuesSearch,
    },
    /// Find metadata for creating or editing issues
    Meta {
        #[structopt(flatten)]
        opts: meta::IssueMetadata,
    },
    /// Create new issues
    Create {
        /// The data to populate in the new issue
        ///
        /// This option is aware of two special values
        /// '-' will be treated as stdin
        /// '@<pathspec>' will be treated as a filename to read the data from
        #[structopt(short, long, value_name = "DATA")]
        data: String,

        #[structopt(flatten)]
        opts: create::IssueCreate,
    },
    /// Edit existing issues
    Edit {
        #[structopt(value_name = "KEY/ID")]
        key: String,

        /// The data to change in the given issue
        ///
        /// This option is aware of two special values
        /// '-' will be treated as stdin
        /// '@<pathspec>' will be treated as a filename to read the data from
        #[structopt(short, long, value_name = "DATA")]
        data: String,
    },
}
