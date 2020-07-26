use {
    reqwest::{Error as HttpError, StatusCode},
    serde::{Deserialize, Serialize},
    serde_json::{to_writer as json, to_writer_pretty as json_pretty},
    std::{collections::HashMap, fmt, io},
    thiserror::Error,
    url::ParseError,
};

#[derive(Debug, Error)]
pub enum InitError {
    #[error(transparent)]
    BadUrl(#[from] ParseError),
    #[error("Unable to parse a valid host from: '{}'", .0)]
    InvalidHost(String),
}

#[derive(Debug, Error)]
pub enum JiraError {
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("JIRA client fault [{}]:\n{:#}", .0.code, .0.errors)]
    Fault(ClientFault),
}

#[derive(Debug)]
pub struct ClientFault {
    pub code: StatusCode,
    pub errors: ApiError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiError {
    #[serde(rename = "errorMessages")]
    pub messages: Vec<String>,
    pub errors: HashMap<String, String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty = f.alternate();
        let mut bridge = WriteBridge { inner: f };

        if pretty {
            // {:#}
            json_pretty(&mut bridge, self).map_err(|_| fmt::Error)
        } else {
            // {}
            json(&mut bridge, self).map_err(|_| fmt::Error)
        }
    }
}

/// Bridge impl between `io::write` & `fmt::Formatter`
struct WriteBridge<'a, 'b: 'a> {
    inner: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> io::Write for WriteBridge<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        fn io_error<E>(_: E) -> io::Error {
            // Error value does not matter because fmt::Display just maps it to fmt::Error
            io::Error::new(io::ErrorKind::Other, "...")
        }
        let s = std::str::from_utf8(buf).map_err(io_error)?;
        self.inner.write_str(s).map_err(io_error)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
