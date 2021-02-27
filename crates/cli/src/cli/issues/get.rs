use {super::*, IssueOptions::Get};

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
    #[structopt(
        short,
        long = "field",
        number_of_values = 1,
        set = ArgSettings::AllowLeadingHyphen
    )]
    pub fields: Option<Vec<String>>,

    /// List of expands to return in the response
    ///
    /// Possible values
    /// 'renderedFields', 'names', 'schema', 'transitions', 'editmeta', 'changelog', 'versionedRepresentations'
    #[structopt(short, long, number_of_values = 1)]
    pub expand: Option<Vec<String>>,

    /// List of properties to return
    ///
    /// Special
    /// '*all' will return all properties
    ///
    /// Modifiers
    /// '-' A dash prefixed to any non special property will omit the property
    #[structopt(
        short,
        long = "property",
        number_of_values = 1,
        set = ArgSettings::AllowLeadingHyphen
    )]
    pub properties: Option<Vec<String>>,
}

impl<'a> Into<Get> for &'a IssuesGet {
    fn into(self) -> Get {
        let cli = &self;
        Get::new().with(|this| {
            this.fields_by_key(cli.fields_by_key)
                .update_history(cli.update_history)
                .fields(cli.fields.as_ref().into_iter().flatten())
                .expand(cli.expand.as_ref().into_iter().flatten())
                .properties(cli.properties.as_ref().into_iter().flatten())
        })
    }
}
