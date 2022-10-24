use crate::spec::BRASIL_API_URL;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DddError {
    message: String,
    name: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Errored {
    NotFound(DddError),
    Unexpected,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnexpectedError {
    pub code: u16,
    pub message: String,
    pub error: Errored,
}

pub async fn get_ddd(ddd: &str) -> Result<Ddd, UnexpectedError> {
    let url = format!("{}/api/ddd/v1/{}", BRASIL_API_URL, ddd);

    let response = reqwest::get(&url).await.unwrap();
    let status = response.status().as_u16();

    if status != 200 {
        let error: DddError = serde_json::from_str(&response.text().await.unwrap()).unwrap();

        return Err(UnexpectedError {
            code: status,
            message: error.clone().message,
            error: Errored::NotFound(error),
        });
    }

    let ddd: Ddd = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(ddd)
}

pub async fn ddd_exists(ddd: &str) -> Result<bool, UnexpectedError> {
    let url = format!("{}/api/ddd/v1/{}", BRASIL_API_URL, ddd);

    let response = reqwest::get(&url).await.unwrap();
    let status = response.status().as_u16();

    if status == 404 {
        Ok(false)
    } else if status == 200 {
        Ok(true)
    } else {
        let error: DddError = serde_json::from_str(&response.text().await.unwrap()).unwrap();

        Err(UnexpectedError {
            code: status,
            message: error.clone().message,
            error: Errored::NotFound(error),
        })
    }
}

#[cfg(test)]
mod ddd_tests {
    use super::*;

    #[tokio::test]
    async fn get_ddd_test() {
        let cep = get_ddd("61").await.unwrap();

        assert_eq!(cep.state, "DF");
    }

    #[tokio::test]
    async fn get_ddd_error() {
        let cep = get_ddd("123").await;

        assert!(cep.is_err());
    }

    #[tokio::test]
    async fn ddd_exists_test() {
        let cep = ddd_exists("21").await.unwrap();

        assert!(cep);
    }
}
