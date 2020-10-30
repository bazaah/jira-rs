mod cli;

use crate::cli::{CliOptions, Command, Issues as IssuesCmd, MetaKind};
use {
    anyhow::{anyhow, Result},
    jira_rs::{client::Jira, issue},
    json::{to_writer_pretty as json_pretty, value::RawValue as RawJson},
    serde_json as json,
    std::{convert::TryFrom, io::stdout, path::PathBuf},
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
                let options = opts.as_options().with(|this| this.jql(jql));

                let search = client.issues().search(Some(&options)).await?;

                json_pretty(stdout(), &search)?;
            }
            IssuesCmd::Create { ref data, ref opts } => {
                let options: issue::options::Create = opts.into();
                let data = DataSpec::try_from(data)?.to_json().await?;

                let created = client.issues().create(&data, Some(&options)).await?;

                json_pretty(stdout(), &created)?;
            }
            IssuesCmd::Edit { ref key, ref data } => {
                let data = DataSpec::try_from(data)?.to_json().await?;

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

#[derive(Debug, Clone)]
enum DataSpec {
    Text(String),
    FilePath(PathBuf),
    Stdin,
}

impl DataSpec {
    pub async fn to_json(self) -> Result<Box<RawJson>> {
        use tokio::prelude::*;
        match self {
            Self::Text(ref data) => Ok(json::from_str(data)?),
            Self::FilePath(path) => Ok(json::from_slice(tokio::fs::read(&path).await?.as_ref())?),
            Self::Stdin => {
                let mut data = Vec::new();
                tokio::io::stdin().read_to_end(&mut data).await?;

                Ok(json::from_slice(&data)?)
            }
        }
    }
}

impl TryFrom<&str> for DataSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "-" => Ok(Self::Stdin),
            maybe_file => Ok(maybe_file
                .strip_prefix("@")
                .map(|path| Self::FilePath(PathBuf::from(path)))
                .unwrap_or_else(|| Self::Text(maybe_file.to_string()))),
        }
    }
}

impl TryFrom<&String> for DataSpec {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        DataSpec::try_from(value.as_str())
    }
}
