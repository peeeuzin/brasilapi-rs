use crate::{error::*, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bank {
    pub ispb: String,
    pub name: Option<String>,
    pub code: Option<i32>,

    #[serde(rename = "fullName")]
    pub fullname: Option<String>,
}

pub struct BankService {
    base_url: String,
}

impl BankService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get_all_banks(&self) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/banks/v1", self.base_url);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }

    async fn get_bank_by_code(&self, code: i32) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/banks/v1/{}", self.base_url, code);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }
}

/// ## `get_all_banks()`
/// Lista todos os bancos cadastrados.
///
///
/// ### Retorno
/// * `Result<Vec<Bank>, Error>`
///
/// # Exemplo
///
/// ```rust
/// use brasilapi::bank::{self, Bank};
///
/// #[tokio::main]
/// async fn main() {
///    let banks:Vec<Bank> = bank::get_all_banks().await.unwrap();
/// }
pub async fn get_all_banks() -> Result<Vec<Bank>, Error> {
    let bank_service = BankService::new(BRASIL_API_URL);

    let response = bank_service.get_all_banks().await?;

    let body = response.text().await.unwrap();
    let banks: Vec<Bank> = serde_json::from_str(&body).unwrap();

    Ok(banks)
}

/// ## `get_bank(code: i32)`
/// Consulta um banco pelo seu código identificador
///
/// ### Argumentos
/// * `code:i32` => Código do banco.
///
/// ### Retorno:
/// * `Result<Bank, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::bank::{self, Bank};
///
/// #[tokio::main]
/// async fn main() {
///   let bank:Bank = bank::get_bank(1).await.unwrap();
/// }
/// ```
pub async fn get_bank(code: i32) -> Result<Bank, Error> {
    let bank_service = BankService::new(BRASIL_API_URL);

    let response = bank_service.get_bank_by_code(code).await?;

    let body = response.text().await.unwrap();
    let bank: Bank = serde_json::from_str(&body).unwrap();

    Ok(bank)
}

#[cfg(test)]
mod bank_tests {
    use super::*;

    #[tokio::test]
    async fn get_bank_test() {
        let banks = get_all_banks().await.unwrap();

        let bank = get_bank(1).await.unwrap();

        assert!(banks.contains(&bank));
    }

    #[tokio::test]
    async fn get_bank_error() {
        let bank = get_bank(2).await;

        assert!(bank.is_err());
    }
}
