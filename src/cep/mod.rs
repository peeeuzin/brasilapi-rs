use crate::{error::*, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

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
    pub errors: Vec<Error>,
}

pub struct CepService {
    base_url: String,
}

impl CepService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get_cep_request(&self, cep_code: &str) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/cep/v2/{}", self.base_url, cep_code);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }

    async fn validate_cep(&self, cep_code: &str) -> Result<bool, Error> {
        let response = self.get_cep_request(cep_code).await;

        match response {
            Ok(_) => Ok(true),
            Err(e) => match e.code {
                Some(404) => Ok(false),
                _ => Err(e),
            },
        }
    }
}

/// #### `get_cep(cep_code: &str)`
/// Busca por CEP com múltiplos providers de fallback.
///
/// A busca utiliza como fonte principal o OpenCep, caso não encontre o CEP é buscado em diversos outros providers de CEP.
///
/// ### Argumento
/// * `cep_code:&str` => CEP para ser consultado.
///
/// ### Retorno
/// * `Result<Cep, Error>`
///
/// # Exemplo
/// ```
/// use brasilapi::cep;
///
/// #[tokio::main]
/// async fn main() {
///    let cep = cep::get_cep("01001000").await.unwrap();
/// }
/// ```
pub async fn get_cep(cep_code: &str) -> Result<Cep, Error> {
    let cep_service = CepService::new(BRASIL_API_URL);

    let response = cep_service.get_cep_request(cep_code).await?;

    let body = response.text().await.unwrap();
    let cep: Cep = serde_json::from_str(&body).unwrap();

    Ok(cep)
}

/// #### `validate(cep_code: &str)`
/// Valida um CEP.
///
/// ### Argumento
/// * `cep_code:&str` => CEP para ser validado.
///
/// Retorno
/// * `Result<bool, Error>`
///
/// # Exemplo
/// ```
/// use brasilapi::cep;
///
/// #[tokio::main]
/// async fn main() {
///   let is_valid = cep::validate("01001000").await.unwrap();  
/// }
pub async fn validate(cep_code: &str) -> Result<bool, Error> {
    let cep_service = CepService::new(BRASIL_API_URL);
    cep_service.validate_cep(cep_code).await
}

#[cfg(test)]
mod cep_tests {
    use super::*;
    use httpmock::MockServer;
    use reqwest::StatusCode;
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
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path(format!("/api/cep/v2/{cep_code}"));
                then.status(500).json_body(json!({
                    "name": "Internal Server Error",
                    "message": "Error interno do servidor",
                }));
            })
            .await;

        let cep_service = CepService::new(&server.base_url());
        let response = cep_service.get_cep_request(cep_code).await;
        let expectation = response.unwrap_err();

        mock.assert_async().await;

        assert_eq!(
            expectation.code,
            Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16())
        );
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
        let mock = server
            .mock_async(|when, then| {
                when.method("GET").path(format!("/api/cep/v2/{cep_code}"));
                then.status(500).json_body(json!({
                "name": "Internal Server Error",
                "message": "Error interno do servidor",
                }));
            })
            .await;

        let cep_service = CepService::new(&server.base_url());
        let response = cep_service.get_cep_request(cep_code).await;
        let expectation = response.unwrap_err();

        mock.assert_async().await;

        assert_eq!(
            expectation.code,
            Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16())
        );
    }
}
