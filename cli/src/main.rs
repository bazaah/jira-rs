mod cli;

use std::io::Read;

use crate::cli::{CliOptions, Command, Issues as IssuesCmd, MetaKind};
use {
    anyhow::{anyhow, Result},
    jira_rs::{client::Jira, issue},
    json::{to_writer_pretty as json_pretty, value::RawValue as RawJson},
    serde_json as json,
    std::io::stdout,
};

#[tokio::main(worker_threads = 2)]
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
                let key = to_string(key)?;

                let options = opts.into();
                let issue = client.issues().get(key, Some(&options)).await?;

                let stdout = stdout();
                json_pretty(stdout, &issue)?;
            }
            IssuesCmd::Search { ref jql, ref opts } => {
                let jql = to_string(jql)?;

                let options = opts.as_options().with(|this| this.jql(jql));

                let search = client.issues().search(Some(&options)).await?;

                json_pretty(stdout(), &search)?;
            }
            IssuesCmd::Create { ref data, ref opts } => {
                let options: issue::options::Create = opts.into();
                let data = to_json(data)?;

                let created = client.issues().create(&data, Some(&options)).await?;

                json_pretty(stdout(), &created)?;
            }
            IssuesCmd::Edit { ref key, ref data } => {
                let data = to_json(data)?;

                client.issues().edit(key, &data).await?;

                json_pretty(
                    stdout(),
                    &json::json!({key: "Successfully updated", "data": data}),
                )?;
            }
            IssuesCmd::Meta { ref opts } => match MetaKind::from(opts) {
                // User provided a specific issue
                MetaKind::Edit(issue) => {
                    let meta_edit = client.issues().meta_edit(issue).await?;

                    json_pretty(stdout(), &meta_edit)?;
                }
                // No issue, run a query based on options passed in
                MetaKind::Create(options) => {
                    let meta_create = client.issues().meta_create(Some(&options)).await?;

                    json_pretty(stdout(), &meta_create)?;
                }
            },
        },
    }

    Ok(())
}

fn to_json(i: &grab::Input) -> Result<Box<RawJson>> {
    let src = i.access()?;

    let json = json::from_reader(src)?;

    Ok(json)
}

fn to_string(i: &grab::Input) -> Result<String> {
    let mut src = i.access()?;

    let mut s = String::new();
    src.read_to_string(&mut s)?;

    Ok(s)
}
