use crate::spec::BRASIL_API_URL;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bank {
    pub ispb: String,
    pub name: String,
    pub code: Option<i32>,

    #[serde(rename = "fullName")]
    pub fullname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BankError {
    pub name: Option<String>,
    pub message: String,

    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnexpectedError {
    pub code: u16,
    pub message: String,
    pub error: Errored,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Errored {
    Unexpected,
    NotFound(BankError),
}

pub async fn get_all_banks() -> Result<Vec<Bank>, UnexpectedError> {
    let url = format!("{}/api/banks/v1", BRASIL_API_URL);

    let response = reqwest::get(&url).await.unwrap();
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

pub async fn get_bank(code: i32) -> Result<Bank, UnexpectedError> {
    let url = format!("{}/api/banks/v1/{}", BRASIL_API_URL, code);

    let response = reqwest::get(&url).await.unwrap();
    let status = response.status().as_u16();

    if status == 404 {
        let body = response.text().await.unwrap();
        let error: BankError = serde_json::from_str(&body).unwrap();

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
    async fn get_ddd_error() {
        let bank = get_bank(2).await;

        assert!(bank.is_err());
    }
}
