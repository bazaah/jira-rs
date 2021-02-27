use {
    super::*,
    IssueOptions::{Search, ValidateQuery},
};

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
    #[structopt(short, long, number_of_values = 1)]
    pub expand: Option<Vec<String>>,

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
    #[structopt(
        short,
        long = "property",
        number_of_values = 1,
        set = ArgSettings::AllowLeadingHyphen
    )]
    pub properties: Option<Vec<String>>,

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
    pub validate: Option<ValidateQuery>,
}

impl IssuesSearch {
    pub fn as_options(&self) -> Search {
        self.into()
    }
}

impl<'a> Into<Search> for &'a IssuesSearch {
    fn into(self) -> Search {
        let cli = &self;
        Search::new().with(|this| {
            this.fields_by_key(cli.fields_by_key)
                .expand(cli.expand.as_ref().into_iter().flatten())
                .fields(cli.fields.as_ref().into_iter().flatten())
                .max_results(cli.max_results)
                .properties(cli.properties.as_ref().into_iter().flatten())
                .start_at(cli.start_at)
                .validate(cli.validate)
        })
    }
}

fn try_into_validate(input: &str) -> Result<ValidateQuery, String> {
    ValidateQuery::try_new(input).ok_or_else(|| {
        format!(
            "expected one of [{}], got '{}'",
            "strict, warn, none", input
        )
    })
}
