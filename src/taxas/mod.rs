use serde::{Deserialize, Serialize};

use crate::{error::Error, spec::BRASIL_API_URL};

#[derive(Debug, Serialize, Deserialize)]
pub struct Taxa {
    pub nome: String,
    pub valor: f64,
}

pub struct TaxasService {
    base_url: String,
}

impl TaxasService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get_taxa_request(&self, sigla: &str) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/taxas/v1/{}", self.base_url, sigla);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }

    async fn list_taxas_request(&self) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/taxas/v1", self.base_url);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }
}

/// #### `get_taxa(sigla: &str)`
/// Busca por uma taxa específica na API BrasilAPI.
///### Argumento
/// * `sigla:&str` => Sigla da taxa para consulta.
///### Retorno
/// * `Result<Taxa, Error>`
/// # Exemplo
/// ```rust
/// use brasilapi::taxas;
///#[tokio::main]
/// async fn main() {
///     let taxa = taxas::get_taxa("CDI").await.unwrap();
///     println!("Nome: {}", taxa.nome);
///     println!("Valor: {}", taxa.valor);
/// }
/// ```
pub async fn get_taxa(sigla: &str) -> Result<Taxa, Error> {
    let service = TaxasService::new(BRASIL_API_URL);
    let response = service.get_taxa_request(sigla).await?;

    let body = response.text().await.unwrap();
    let taxa: Taxa = serde_json::from_str(&body).unwrap();

    Ok(taxa)
}

/// #### `list_taxas()`
/// Lista todas as taxas disponíveis na API BrasilAPI.
///### Retorno
/// * `Result<Vec<Taxa>, Error>`
/// # Exemplo
/// ```rust
/// use brasilapi::taxas;
/// #[tokio::main]
/// async fn main() {
///     let taxas = taxas::list_taxas().await.unwrap();
/// }
/// ```
pub async fn list_taxas() -> Result<Vec<Taxa>, Error> {
    let service = TaxasService::new(BRASIL_API_URL);
    let response = service.list_taxas_request().await?;

    let body = response.text().await.unwrap();
    let taxas: Vec<Taxa> = serde_json::from_str(&body).unwrap();

    Ok(taxas)
}

#[cfg(test)]
mod taxas_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_taxa() {
        let taxa = get_taxa("CDI").await.unwrap();
        assert_eq!(taxa.nome, "CDI");
        assert!(taxa.valor > 0.0);
    }

    #[tokio::test]
    async fn test_list_taxas() {
        let taxas = list_taxas().await.unwrap();
        assert!(!taxas.is_empty());
        assert!(taxas.iter().any(|t| t.nome == "CDI"));
    }
}
