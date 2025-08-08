use crate::{error::*, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    pub status_code: u16,
    pub status: String,
    pub fqdn: String,
    pub suggestions: Option<Vec<String>>,
    pub hosts: Option<Vec<String>>,
    #[serde(rename = "publication-status")]
    pub publication_status: Option<String>,
    #[serde(rename = "expires-at")]
    pub expires_at: Option<String>,
    pub reasons: Option<Vec<String>>,
}

pub struct RegistroBrService {
    base_url: String,
}

impl RegistroBrService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get_domain_by_name(&self, name: &str) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/registrobr/v1/{}", self.base_url, name);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }
}

/// #### `get_domain_by_name(name: &str)`
/// Retorna informações sobre um domínio.
///
/// ### Argumento
/// * `name:&str` => Nome do domínio para consulta.
///
/// ### Retorno
/// * `Result<Domain, Error>`
///
/// # Exemplo
/// ```
/// use brasilapi::registrobr;
///
/// #[tokio::main]
/// async fn main() {
///    let domain = registrobr::get_domain_by_name("google.com").await.unwrap();
/// }
/// ```
pub async fn get_domain_by_name(name: &str) -> Result<Domain, Error> {
    let registro_br_service = RegistroBrService::new(BRASIL_API_URL);

    let response = registro_br_service.get_domain_by_name(name).await?;

    let body = response.text().await.unwrap();
    let domain: Domain = serde_json::from_str(&body).unwrap();

    Ok(domain)
}

#[cfg(test)]
mod registrobr_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_domain_by_name() {
        let domain = get_domain_by_name("google.com").await.unwrap();

        assert_eq!(domain.status_code, 2);
        assert_eq!(domain.status, "REGISTERED");
        assert_eq!(domain.fqdn, "google.com.br");
    }
}
