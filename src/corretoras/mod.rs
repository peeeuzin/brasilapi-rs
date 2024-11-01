use serde::{Deserialize, Serialize};

use crate::{error::Error, spec::BRASIL_API_URL};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Corretora {
    cnpj: String,
    nome_social: String,
    nome_comercial: String,
    bairro: String,
    cep: String,
    codigo_cvm: String,
    complemento: String,
    data_inicio_situacao: String,
    data_patrimonio_liquido: String,
    data_registro: String,
    email: String,
    logradouro: String,
    municipio: String,
    pais: String,
    telefone: String,
    uf: String,
    valor_patrimonio_liquido: String,
}

pub struct CorretorasService {
    url: String,
}

impl CorretorasService {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    async fn get_corretoras_request(&self) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/cvm/corretoras/v1", self.url);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }

    async fn get_corretora_by_cnpj(&self, cnpj: &str) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/cvm/corretoras/v1/{}", self.url, cnpj);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }
}

/// #### `get_corretoras()`
/// Retorna as corretoras nos arquivos da CVM.
///
/// ### Retorno
/// * `Result<Vec<Corretora>, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::corretoras::{self, Corretora};
///
/// #[tokio::main]
/// async fn main() {
///    let corretoras: Vec<Corretora> = corretoras::get_corretoras().await.unwrap();
/// }
///
/// ```
pub async fn get_corretoras() -> Result<Vec<Corretora>, Error> {
    let corretoras_service = CorretorasService::new(BRASIL_API_URL);

    let response = corretoras_service.get_corretoras_request().await?;

    let body = response.text().await.unwrap();
    let corretoras: Vec<Corretora> = serde_json::from_str(&body).unwrap();

    Ok(corretoras)
}

/// #### `get_corretora(cnpj: &str)`
/// Retorna uma corretora a partir de um CNPJ nos arquivos da CVM.
///
/// ### Argumento
/// * `cnpj:&str` => CNPJ da corretora.
///
/// ### Retorno
/// * `Result<Corretora, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::corretoras::{self, Corretora};
///
/// #[tokio::main]
/// async fn main() {
///   let corretora: Corretora = corretoras::get_corretora("02332886000104").await.unwrap();
/// }
///
/// ```
pub async fn get_corretora(cnpj: &str) -> Result<Corretora, Error> {
    let corretoras_service = CorretorasService::new(BRASIL_API_URL);

    let response = corretoras_service.get_corretora_by_cnpj(cnpj).await?;

    let body = response.text().await.unwrap();
    let corretora: Corretora = serde_json::from_str(&body).unwrap();

    Ok(corretora)
}

#[cfg(test)]
mod corretoras_tests {
    use super::*;

    #[tokio::test]
    async fn get_corretoras_test() {
        let corretoras = get_corretoras().await.unwrap();

        assert!(!corretoras.is_empty());
    }

    #[tokio::test]
    async fn get_corretora_test() {
        let corretora = get_corretora("02332886000104").await.unwrap();
        assert_eq!(corretora.cnpj, "02332886000104");
        assert_eq!(corretora.nome_social, "XP INVESTIMENTOS CCTVM S.A.");
    }
}
