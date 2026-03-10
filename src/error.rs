use std::fmt;

#[derive(Debug)]
pub enum ThreadsError {
    Http(reqwest::Error),
    Api {
        code: Option<i64>,
        message: String,
    },
}

impl fmt::Display for ThreadsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreadsError::Http(e) => write!(f, "HTTP error: {e}"),
            ThreadsError::Api { code, message } => {
                if let Some(code) = code {
                    write!(f, "API error {code}: {message}")
                } else {
                    write!(f, "API error: {message}")
                }
            }
        }
    }
}

impl std::error::Error for ThreadsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ThreadsError::Http(e) => Some(e),
            ThreadsError::Api { .. } => None,
        }
    }
}

impl From<reqwest::Error> for ThreadsError {
    fn from(e: reqwest::Error) -> Self {
        ThreadsError::Http(e)
    }
}

pub type Result<T> = std::result::Result<T, ThreadsError>;
