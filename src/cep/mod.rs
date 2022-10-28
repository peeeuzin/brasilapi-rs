use crate::spec::BRASIL_API_URL;
use serde::{Deserialize, Serialize};
use reqwest::StatusCode;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cep {
    pub cep: String,
    pub state: String,
    pub city: String,
    pub neighborhood: String,
    pub street: String,
    pub service: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "type")]
    pub kind: String,
    pub coordinates: Coordinates,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub longitude: String,
    pub latitude: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CepError {
    pub name: String,
    pub message: String,

    #[serde(rename = "type")]
    pub kind: String,
    pub errors: Vec<CepServiceError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CepServiceError {
    pub name: String,
    pub message: String,
    pub service: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Errored {
    NotFound(CepError),
    Unexpected,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnexpectedError {
    pub code: u16,
    pub message: String,
    pub error: Errored,
}

pub struct CepService {
    base_url: String
}

impl CepService {
    fn new(base_url: &str) -> CepService {
        CepService {
            base_url: base_url.to_string(),
        }
    }

    async fn get_cep_request(&self, cep_code: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/cep/v2/{}", self.base_url, cep_code);
        reqwest::get(&url).await
    }

    async fn dispatch(&self, response: reqwest::Response, cep_code: &str) -> Result<Cep, UnexpectedError> {
        match response.status() {
            StatusCode::OK => {
                let body = response.text().await.unwrap();
                let cep: Cep = serde_json::from_str(&body).unwrap();

                Ok(cep)
            }
            StatusCode::NOT_FOUND => {
                let body = response.text().await.unwrap();
                let cep_err: CepError = serde_json::from_str(&body).unwrap();

                Err(UnexpectedError {
                    code: 404,
                    message: format!("Not founded cep: {}", cep_code),
                    error: Errored::NotFound(cep_err),
                })
            }
            code => Err(UnexpectedError {
                code: code.as_u16(),
                message: format!("Unexpected error with code: {}", code),
                error: Errored::Unexpected,
            })
        }
    }

    async fn validate(&self, response: reqwest::Response, cep_code: &str) -> Result<bool, UnexpectedError> {
        match response.status() {
            StatusCode::OK => Ok(true),
            code => Err(UnexpectedError {
                code: code.as_u16(),
                message: format!("Unexpected error with code: {}", code),
                error: Errored::Unexpected,
            }),
        }
    }
}

/// Faz um GET Request para a API de CEP do Brasil API e retorna o CEP mais detalhado possível.
///
/// Argumentos:
///
/// * `cep_code`: CEP para ser consultado
///
/// Retorna:
///
/// Result<Cep, UnexpectedError>
pub async fn get_cep(cep_code: &str) -> Result<Cep, UnexpectedError> {
    let cep_service = CepService::new(BRASIL_API_URL);
    let response = cep_service.get_cep_request(cep_code).await.unwrap();
    cep_service.dispatch(response, cep_code).await
}

/// Faz um GET Request para a API de CEP do Brasil API e verifica se o CEP é valido (não é preciso se é válido ou não)
///
/// Argumentos:
///
/// * `cep_code`: CEP a ser validado
///
/// Retorna:
///
/// Um resultado com valor booleano indicando se o CEP é válido ou não ou o mapeamento do erro.
pub async fn validate(cep_code: &str) -> Result<bool, UnexpectedError> {
    let cep_service = CepService::new(BRASIL_API_URL);
    let response = cep_service.get_cep_request(cep_code).await.unwrap();
    cep_service.validate(response, cep_code).await
}

#[cfg(test)]
mod cep_tests {
    use super::*;
    use httpmock::MockServer;
    use serde_json::json;

    #[tokio::test]
    async fn get_cep_test() {
        let cep = get_cep("01001000").await.unwrap();

        assert_eq!(cep.state, "SP");
        assert_eq!(cep.street, "Praça da Sé");
    }

    #[tokio::test]
    async fn get_cep_error() {
        let cep = get_cep("12345678").await;

        assert!(cep.is_err());
    }

    #[tokio::test]
    async fn get_cep_unexpected_error() {
        let cep_code = "99999999";
        let server = MockServer::start_async().await;
        let mock = server.mock_async(|when, then| {
            when.method("GET")
                .path(format!("/api/cep/v2/{}", cep_code));
            then.status(500)
                .json_body(json!({
                    "name": "Internal Server Error",
                    "message": "Error interno do servidor",
                }));
        })
        .await;

        let cep_service = CepService::new(&server.base_url());
        let response = cep_service.get_cep_request(cep_code);
        let result = cep_service.dispatch(response.await.unwrap(), cep_code);
        let expectation = result.await.unwrap_err();

        mock.assert_async().await;

        assert_eq!(expectation.code, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
    }

    #[tokio::test]
    async fn validate_test() {
        let cep = validate("01001000").await.unwrap();

        assert!(cep);
    }

    #[tokio::test]
    async fn get_validate_unexpected_error() {
        let cep_code = "99999998";
        let server = MockServer::start_async().await;
        let mock = server.mock_async(|when, then| {
            when.method("GET")
            .path(format!("/api/cep/v2/{}", cep_code));
            then.status(500)
            .json_body(json!({
            "name": "Internal Server Error",
            "message": "Error interno do servidor",
            }));
        })
        .await;

        let cep_service = CepService::new(&server.base_url());
        let response = cep_service.get_cep_request(cep_code);
        let result = cep_service.validate(response.await.unwrap(), cep_code);
        let expectation = result.await.unwrap_err();

        mock.assert_async().await;

        assert_eq!(expectation.code, StatusCode::INTERNAL_SERVER_ERROR.as_u16());
    }
}
