use {
    crate::error::{InitError, JiraError},
    reqwest::{Client, Method, Request, RequestBuilder},
    std::sync::Arc,
    url::{Position, Url},
};

#[derive(Debug, Clone)]
pub enum Authentication {
    Basic(Arc<str>, Arc<str>),
}

impl Authentication {
    pub fn basic<U, P>(username: U, password: P) -> Self
    where
        U: AsRef<str>,
        P: AsRef<str>,
    {
        Self::Basic(Arc::from(username.as_ref()), Arc::from(password.as_ref()))
    }

    pub fn authorize(&self, request: RequestBuilder) -> Result<RequestBuilder, JiraError> {
        match self {
            Self::Basic(user, password) => Ok(request.basic_auth(user, Some(password))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Jira {
    agent: Client,
    auth: Authentication,
    remote: Arc<Url>,
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
        let base = &Url::parse(host.as_ref())?[..Position::BeforePath];
        let remote = Url::parse(base)?.join("/rest/api/2/")?;

        Ok(Self {
            agent: client,
            auth: auth.into(),
            remote: remote.into(),
        })
    }

    fn get<F>(&self, handler: F) -> Result<Request, JiraError>
    where
        F: FnMut(RequestBuilder) -> Result<RequestBuilder, JiraError>,
    {
        self.generate(Method::GET, handler)
    }

    fn post<F>(&self, handler: F) -> Result<Request, JiraError>
    where
        F: FnMut(RequestBuilder) -> Result<RequestBuilder, JiraError>,
    {
        self.generate(Method::POST, handler)
    }

    fn put<F>(&self, handler: F) -> Result<Request, JiraError>
    where
        F: FnMut(RequestBuilder) -> Result<RequestBuilder, JiraError>,
    {
        self.generate(Method::PUT, handler)
    }

    fn delete<F>(&self, handler: F) -> Result<Request, JiraError>
    where
        F: FnMut(RequestBuilder) -> Result<RequestBuilder, JiraError>,
    {
        self.generate(Method::DELETE, handler)
    }

    fn generate<F>(&self, method: Method, handler: F) -> Result<Request, JiraError>
    where
        F: FnMut(RequestBuilder) -> Result<RequestBuilder, JiraError>,
    {
        let mut handler = handler;
        let request = handler(self.agent.request(method, self.remote.as_str()))?;
        let request = self.auth.authorize(request)?;

        request.build().map_err(Into::into)
    }
}
