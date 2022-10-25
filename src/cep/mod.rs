use crate::spec::BRASIL_API_URL;
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

pub struct CepService;

impl CepService {
    async fn get_cep_request(cep_code: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/cep/v2/{}", BRASIL_API_URL, cep_code);
        reqwest::get(&url).await
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
    let response = CepService::get_cep_request(cep_code).await.unwrap();

    match response.status().as_u16() {
        200 => {
            let body = response.text().await.unwrap();
            let cep: Cep = serde_json::from_str(&body).unwrap();

            Ok(cep)
        }
        404 => {
            let body = response.text().await.unwrap();
            let cep_err: CepError = serde_json::from_str(&body).unwrap();

            Err(UnexpectedError {
                code: 404,
                message: format!("Not founded cep: {}", cep_code),
                error: Errored::NotFound(cep_err),
            })
        }
        code => Err(UnexpectedError {
            code,
            message: format!("Unexpected error with code: {}", code),
            error: Errored::Unexpected,
        }),
    }
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
    let response = CepService::get_cep_request(cep_code).await.unwrap();

    match response.status().as_u16() {
        200 => Ok(true),
        code => Err(UnexpectedError {
            code,
            message: format!("Unexpected error with code: {}", code),
            error: Errored::Unexpected,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    async fn validate_test() {
        let cep = validate("01001000").await.unwrap();

        assert!(cep);
    }
}
