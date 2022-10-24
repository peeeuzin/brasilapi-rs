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

/// Faz um GET Request para a API de CEP do Brasil API e retorna o CEP mais detalhado possível.
///
/// Argumentos:
///
/// * `cep_code`: CEP para ser consultado
///
/// Retorna:
///
/// Result<Cep, CepError>
pub async fn get_cep(cep_code: &str) -> Result<Cep, CepError> {
    let url = format!("{}/api/cep/v2/{}", BRASIL_API_URL, cep_code);

    let response = reqwest::get(&url).await.unwrap();

    if response.status().as_u16() == 404 {
        let body = response.text().await.unwrap();
        let cep: CepError = serde_json::from_str(&body).unwrap();

        Err(cep)
    } else {
        let body = response.text().await.unwrap();
        let cep: Cep = serde_json::from_str(&body).unwrap();

        Ok(cep)
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
/// Um valor booleano indicando se o CEP é válido ou não.
pub async fn validate(cep_code: &str) -> bool {
    let url = format!("{}/api/cep/v2/{}", BRASIL_API_URL, cep_code);

    let response = reqwest::get(&url).await.unwrap();

    response.status().as_u16() == 200
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
        let cep = validate("01001000").await;

        assert!(cep);
    }
}
