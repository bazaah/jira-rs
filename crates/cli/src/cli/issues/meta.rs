use {super::*, std::str::FromStr, IssueOptions::MetaCreate};

const EXPAND_ALL: &str = "projects.issuetypes.fields";

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

    /// Get the field schema for a single issue
    ///
    /// This option is mutually exclusive with both
    /// --projects and --issue-types which return a range
    /// of matching issue schema(s)
    #[structopt(
        short = "E", long, value_name = "KEY/ID",
        required_unless_one = &["projects", "issue_types"],
        conflicts_with_all = &["projects", "issue_types"]
    )]
    pub edit: Option<String>,

    /// Don't return schemas, only project/issue-type layout
    ///
    /// Useful for exploring possible values for --projects and --issue-types
    #[structopt(short, long)]
    pub short: bool,
}

pub enum MetaKind {
    Create(MetaCreate),
    Edit(String),
}

impl<'a> From<&'a IssueMetadata> for MetaKind {
    fn from(cli: &'a IssueMetadata) -> Self {
        match &cli.edit {
            Some(issue) => Self::Edit(issue.to_string()),
            None => Self::Create(cli.into()),
        }
    }
}

impl<'a> Into<MetaCreate> for &'a IssueMetadata {
    fn into(self) -> MetaCreate {
        let cli = &self;
        let mut meta = MetaCreate::new();

        if let Some(ref projects) = cli.projects {
            for project in projects {
                let _ = match u64::from_str(project) {
                    Ok(num) => meta.project_ids(Some(num)),
                    Err(_) => meta.project_keys(Some(project)),
                };
            }
        }

        if let Some(ref types) = cli.issue_types {
            for ty in types {
                let _ = match u64::from_str(ty) {
                    Ok(num) => meta.issuetype_ids(Some(num)),
                    Err(_) => meta.issuetype_keys(Some(ty)),
                };
            }
        }

        if !cli.short {
            meta.expand(Some(EXPAND_ALL));
        }

        meta
    }
}

/*
impl<'a> Into<MetaCreate> for &'a IssueMetadata {
    fn into(self) -> MetaCreate {
        let opts = MetaCreate::new();

        let opts = try_use(opts, self.projects.as_ref(), |o, v| {
            v.into_iter().fold(o, |o, ref i| match u64::from_str(i) {
                Ok(int) => o.project_ids(Some(Some(int).into_iter())),
                Err(_) => o.project_keys(Some(Some(i.as_str()).into_iter())),
            })
        });

        let opts = try_use(opts, self.issue_types.as_ref(), |o, v| {
            v.into_iter().fold(o, |o, ref i| match u64::from_str(i) {
                Ok(int) => o.issuetype_ids(Some(Some(int).into_iter())),
                Err(_) => o.issuetype_keys(Some(Some(i.as_str()).into_iter())),
            })
        });

        match self.short {
            true => opts,
            false => opts.expand(Some(Some("projects.issuetypes.fields").into_iter())),
        }
    }
}
*/
