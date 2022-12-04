use crate::{error::*, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bank {
    pub ispb: String,
    pub name: String,
    pub code: Option<i32>,

    #[serde(rename = "fullName")]
    pub fullname: String,
}

pub struct BankService;

impl BankService {
    async fn get_all_banks() -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/banks/v1", BRASIL_API_URL);
        reqwest::get(&url).await
    }

    async fn get_bank_by_code(code: i32) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/banks/v1/{}", BRASIL_API_URL, code);
        reqwest::get(&url).await
    }
}

/// Consulta todos os bancos cadastrados na Brasil API
///
/// Retorna:
///
/// Result<Vec<Bank>, UnexpectedError>
pub async fn get_all_banks() -> Result<Vec<Bank>, UnexpectedError> {
    let response = BankService::get_all_banks().await.unwrap();

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
    let banks: Vec<Bank> = serde_json::from_str(&body).unwrap();

    Ok(banks)
}

/// Consulta um banco pelo seu código
///
/// Argumentos:
///
/// * `code`: Código do banco
///
/// Retorna:
///
/// Result<Bank, UnexpectedError>
pub async fn get_bank(code: i32) -> Result<Bank, UnexpectedError> {
    let response = BankService::get_bank_by_code(code).await.unwrap();

    let status = response.status().as_u16();

    if status == 404 {
        let body = response.text().await.unwrap();
        let error: Error = serde_json::from_str(&body).unwrap();

        return Err(UnexpectedError {
            code: status,
            message: error.clone().message,
            error: Errored::NotFound(error),
        });
    } else if status != 200 {
        let body = response.text().await.unwrap();

        return Err(UnexpectedError {
            code: status,
            message: body,
            error: Errored::Unexpected,
        });
    }

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
