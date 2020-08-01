mod cli;

use crate::cli::{CliOptions, StructOpt};
use {
    anyhow::{anyhow, Context, Result},
    jira_rs::{
        client::{Authentication, Jira},
        issue::Issues,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CliOptions::from_args();

    let client = Jira::new(
        cli.host(),
        cli.authentication()
            .ok_or_else(|| anyhow!("Unable to locate authentication"))?,
    )?;

    let issue = client.issues().get("SECCON-4047", None).await?;

    println!("{:#?}", issue);

    Ok(())
}
