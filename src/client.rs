use {
    crate::error::InitError,
    reqwest::Client,
    std::{error::Error, sync::Arc},
    url::Url,
};

#[derive(Debug, Clone)]
pub enum Authentication {
    Basic(Arc<str>, Arc<str>),
}

#[derive(Debug, Clone)]
pub struct Jira {
    agent: Client,
    auth: Authentication,
    remote: Arc<str>,
}

impl Jira {
    pub fn new<H, A>(host: H, auth: A) -> Result<Self, InitError>
    where
        H: AsRef<str>,
        A: Into<Authentication>,
    {
        Self::with_client(host, auth, Client::new())
    }

    pub fn with_client<H, A>(host: H, auth: A, client: reqwest::Client) -> Result<Self, InitError>
    where
        H: AsRef<str>,
        A: Into<Authentication>,
    {
        let url = Url::parse(host.as_ref())?;
        let remote = url
            .host_str()
            .ok_or_else(|| InitError::InvalidHost(url.as_str().into()))?;

        Ok(Self {
            agent: client,
            auth: auth.into(),
            remote: remote.into(),
        })
    }
}
