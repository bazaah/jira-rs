use {super::*, IssueOptions::Create};

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab")]
pub struct IssueCreate {
    /// Should this request update the user's search history?
    ///
    /// Specifically, the user's last.Viewed issue and the
    /// user's recently viewed Projects
    #[structopt(short, long)]
    pub update_history: bool,
}

impl Into<Create> for &IssueCreate {
    fn into(self) -> Create {
        let cli = &self;
        Create::new().with(|this| this.update_history(cli.update_history))
    }
}
