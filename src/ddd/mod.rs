use crate::{error::*, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ddd {
    pub state: String,
    pub cities: Vec<String>,
    pub nome: Option<String>,
    pub regiao: Option<Regiao>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Regiao {
    pub id: i32,
    pub sigla: String,
    pub nome: String,
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

/// #### `get_ddd(ddd: &str)`
/// Retorna estado e lista de cidades por DDD
///
/// ### Argumento
/// * `ddd:&str` => DDD para consulta.
///
/// ### Retorno
/// * `Result<Ddd, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::ddd;
///
/// #[tokio::main]
/// async fn main() {
///   let ddd = ddd::get_ddd("61").await.unwrap();
/// }
/// ```
pub async fn get_ddd(ddd: &str) -> Result<Ddd, Error> {
    let ddd_service = DDDService::new(BRASIL_API_URL);

    let response = ddd_service.get_ddd_request(ddd).await?;

    let body = response.text().await.unwrap();
    let ddd: Ddd = serde_json::from_str(&body).unwrap();

    Ok(ddd)
}

/// #### `ddd_exists(ddd: &str)`
/// Verifica se um DDD existe.
///
/// ### Argumento
/// * `ddd:&str` => DDD para consulta.
///
/// ### Retorno
/// * `Result<bool, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::ddd;
///
/// #[tokio::main]
/// async fn main() {
///     let ddd = ddd::ddd_exists("21").await.unwrap();
/// }
/// ```
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
