use {
    crate::error::{ApiError, ClientFault, InitError, JiraError},
    reqwest::{Client, Method, RequestBuilder, Response},
    serde::de::DeserializeOwned,
    std::sync::Arc,
    url::{PathSegmentsMut, Position, Url},
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

    pub(crate) fn get<F>(&self, endpoint: &[&str], handler: F) -> Result<RequestBuilder, JiraError>
    where
        F: FnMut(RequestBuilder) -> Result<RequestBuilder, JiraError>,
    {
        self.generate(Method::GET, endpoint, handler)
    }

    pub(crate) fn post<F>(&self, endpoint: &[&str], handler: F) -> Result<RequestBuilder, JiraError>
    where
        F: FnMut(RequestBuilder) -> Result<RequestBuilder, JiraError>,
    {
        self.generate(Method::POST, endpoint, handler)
    }

    pub(crate) fn put<F>(&self, endpoint: &[&str], handler: F) -> Result<RequestBuilder, JiraError>
    where
        F: FnMut(RequestBuilder) -> Result<RequestBuilder, JiraError>,
    {
        self.generate(Method::PUT, endpoint, handler)
    }

    pub(crate) fn delete<F>(
        &self,
        endpoint: &[&str],
        handler: F,
    ) -> Result<RequestBuilder, JiraError>
    where
        F: FnMut(RequestBuilder) -> Result<RequestBuilder, JiraError>,
    {
        self.generate(Method::DELETE, endpoint, handler)
    }

    fn generate<F>(
        &self,
        method: Method,
        endpoint: &[&str],
        handler: F,
    ) -> Result<RequestBuilder, JiraError>
    where
        F: FnMut(RequestBuilder) -> Result<RequestBuilder, JiraError>,
    {
        let mut handler = handler;
        let mut base: Url = self.remote.as_ref().clone();
        base.path_segments_mut()
            .expect("Always have a valid pathable URL")
            .extend(endpoint);

        let request = handler(self.agent.request(method, base.as_str()))?;
        let request = self.auth.authorize(request)?;

        Ok(request)
    }
}

pub(crate) async fn parse_response<T>(response: Response) -> Result<T, JiraError>
where
    T: DeserializeOwned,
{
    match response.status() {
        error if error.is_client_error() || error.is_server_error() => {
            Err(JiraError::Fault(ClientFault {
                code: error,
                errors: response.json::<ApiError>().await?,
            }))
        }
        _ => Ok(response.json::<T>().await?),
    }
}