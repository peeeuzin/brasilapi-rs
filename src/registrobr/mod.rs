use crate::{error::*, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    status_code: String,
    status: String,
    fqdn: String,
    suggestions: Option<Vec<String>>,
    hosts: Option<Vec<String>>,
    #[serde(rename = "publication-status")]
    publication_status: Option<String>,
    #[serde(rename = "expires-at")]
    expires_at: Option<String>,
    reasons: Option<Vec<String>>,
}

pub struct RegistroBrService;

impl RegistroBrService {
    async fn get_domain_by_name(name: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/registrobr/v1/{}", BRASIL_API_URL, name);
        reqwest::get(&url).await
    }
}

/// Get a domain by name from Brasil API
///
/// Arguments:
///
/// * `name`: The domain name to be searched.
///
/// Returns:
///
/// A Domain struct
pub async fn get_domain_by_name(name: &str) -> Result<Domain, UnexpectedError> {
    let response = RegistroBrService::get_domain_by_name(name).await.unwrap();

    let status = response.status().as_u16();

    if status != 200 {
        let body = response.text().await.unwrap();

        return Err(UnexpectedError {
            code: status,
            message: body,
            error: Errored::Unexpected,
        });
    }

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

        assert_eq!(domain.status_code, "2");
        assert_eq!(domain.status, "REGISTERED");
        assert_eq!(domain.fqdn, "google.com.br");
    }
}
