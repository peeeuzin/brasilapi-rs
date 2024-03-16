use crate::{error::*, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ddd {
    state: String,
    cities: Vec<String>,
    nome: Option<String>,
    regiao: Option<Regiao>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Regiao {
    id: i32,
    sigla: String,
    nome: String,
}

pub struct DDDService {
    base_url: String,
}

impl DDDService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get_ddd_request(&self, ddd: &str) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/ddd/v1/{}", self.base_url, ddd);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }

    async fn validate_ddd(&self, ddd: &str) -> Result<bool, Error> {
        let response = self.get_ddd_request(ddd).await;

        match response {
            Ok(_) => Ok(true),
            Err(e) => match e.code {
                Some(404) => Ok(false),
                _ => Err(e),
            },
        }
    }
}

/// Retorna um DDD da API do Brasil
///
/// Argumentos:
///
/// * `ddd`: DDD para ser consultado
///
/// Retorna:
///
/// Result<Ddd, UnexpectedError>
pub async fn get_ddd(ddd: &str) -> Result<Ddd, Error> {
    let ddd_service = DDDService::new(BRASIL_API_URL);

    let response = ddd_service.get_ddd_request(ddd).await?;

    let body = response.text().await.unwrap();
    let ddd: Ddd = serde_json::from_str(&body).unwrap();

    Ok(ddd)
}

/// Retorna um booleano indicando se um DDD existe ou não
///
/// Argumentos:
///
/// * `ddd`: DDD a ser validado
///
/// Retorna:
///
/// Result<bool, UnexpectedError>
pub async fn ddd_exists(ddd: &str) -> Result<bool, Error> {
    let ddd_service = DDDService::new(BRASIL_API_URL);

    let response = ddd_service.validate_ddd(ddd).await?;

    Ok(response)
}

#[cfg(test)]
mod ddd_tests {
    use super::*;

    #[tokio::test]
    async fn get_ddd_test() {
        let ddd = get_ddd("61").await.unwrap();

        assert_eq!(ddd.state, "DF");
    }

    #[tokio::test]
    async fn get_ddd_error() {
        let ddd = get_ddd("123").await;

        assert!(ddd.is_err());
    }

    #[tokio::test]
    async fn ddd_exists_test() {
        let ddd = ddd_exists("21").await.unwrap();

        assert!(ddd);
    }
}
