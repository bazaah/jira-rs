use jira_rs::client::Authentication;
pub(crate) use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "jira", rename_all = "kebab_case")]
pub struct CliOptions {
    /// Invoke requests against this host
    #[structopt(env = "JIRA_HOST", short = "H", long)]
    host: String,
    /// Use these authentication credentials for command invocations
    #[structopt(env = "JIRA_AUTH", short = "A", long)]
    auth: String,
    /// Increase log verbosity
    ///
    /// Repeat the command for higher log levels
    #[structopt(short = "v", long = "verbosity", parse(from_occurrences = get_level))]
    debug_level: u8,
}

impl CliOptions {
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
