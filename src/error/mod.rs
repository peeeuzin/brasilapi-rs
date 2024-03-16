use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrasilAPIError {
    pub message: String,
    pub name: Option<String>,

    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Errored {
    NotFound,
    InternalServerError,
    Unexpected,
}

impl Errored {
    pub fn status_code(status_code: Option<StatusCode>) -> Self {
        match status_code {
            Some(StatusCode::NOT_FOUND) => Self::NotFound,
            Some(StatusCode::INTERNAL_SERVER_ERROR) => Self::InternalServerError,
            _ => Self::Unexpected,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub code: Option<u16>,
    pub message: String,
    pub error: Errored,
}

impl Error {
    pub fn new(message: String, error: Errored, code: Option<u16>) -> Self {
        Self {
            code,
            message,
            error,
        }
    }

    pub fn from_error(error: reqwest::Error) -> Self {
        let status = error.status();
        let message = error.to_string();
        let error = Errored::status_code(status);

        Self {
            code: status.map(|s| s.as_u16()),
            message,
            error,
        }
    }

    /// Retorna um erro caso o status code seja diferente de 200
    pub async fn from_response(response: reqwest::Response) -> Result<reqwest::Response, Self> {
        let status = response.status();
        let error = Errored::status_code(Some(status));

        match status {
            reqwest::StatusCode::OK => Ok(response),
            _ => Err(Self {
                code: Some(status.as_u16()),
                message: response.text().await.unwrap(),
                error,
            }),
        }
    }
}
