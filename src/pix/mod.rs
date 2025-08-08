use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Participant {
    pub ispb: String,
    pub nome: String,
    pub nome_reduzido: String,
    pub modalidade_participacao: String,
    pub tipo_participacao: String,
    pub inicio_operacao: String,
}

pub struct PIXService {
    base_url: String,
}

impl PIXService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get_participant_request(&self) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/pix/v1/participants", self.base_url);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }
}

/// #### `get_participants()`
/// Retorna informações de todos os participantes do PIX no dia atual ou anterior
///
/// ### Retorno
/// * `Result<Vec<Participant>, Error>`
///
/// # Exemplo
/// ```
/// use brasilapi::pix::{self, Participant};
///
/// #[tokio::main]
/// async fn main() {
///    let participants: Vec<Participant> = pix::get_participants().await.unwrap();
/// }
/// ```
pub async fn get_participants() -> Result<Vec<Participant>, Error> {
    let pix_service = PIXService::new("https://brasilapi.com.br");

    let response = pix_service.get_participant_request().await?;

    let body = response.text().await.unwrap();
    let participants: Vec<Participant> = serde_json::from_str(&body).unwrap();

    Ok(participants)
}

#[cfg(test)]
mod pix_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_participants() {
        let participants = get_participants().await.unwrap();

        assert!(!participants.is_empty());
    }
}
