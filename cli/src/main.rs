mod cli;

use crate::cli::{CliOptions, Command, Issues as IssuesCmd};
use {
    anyhow::{anyhow, Result},
    jira_rs::{client::Jira, issue},
    serde_json::to_writer_pretty as json_pretty,
    std::io::stdout,
};

#[tokio::main(core_threads = 2)]
async fn main() -> Result<()> {
    let cli = CliOptions::new();

    let client = Jira::new(
        cli.host(),
        cli.authentication()
            .ok_or_else(|| anyhow!("Unable to locate authentication"))?,
    )?;

    match cli.command {
        Command::Issues(cmd) => match cmd {
            IssuesCmd::Get { ref key, ref opts } => {
                let options = opts.into();
                let issue = client.issues().get(key, Some(&options)).await?;

                let stdout = stdout();
                json_pretty(stdout, &issue)?;
            }
            IssuesCmd::Search { ref jql, ref opts } => {
                let options: issue::options::Search = opts.into();

                let search = client
                    .issues()
                    .search(Some(&options.jql(Some(jql))))
                    .await?;

                json_pretty(stdout(), &search)?;
            }
        },
    }

    Ok(())
}
