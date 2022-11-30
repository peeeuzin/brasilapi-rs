use crate::{
    error::{Error, Errored, UnexpectedError},
    spec::BRASIL_API_URL,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Holiday {
    date: String,
    #[serde(rename = "type")]
    kind: String,
    name: String,
    full_name: Option<String>,
}

pub struct HolidayService;

impl HolidayService {
    async fn get_holiday_request(year: &str) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/api/feriados/v1/{}", BRASIL_API_URL, year);

        reqwest::get(&url).await
    }
}

/// ## `get_holidays(year: &str)`
/// Lista os feriados nacionais de determinado ano.
///
/// ### Argumento
/// * `year:&str` => Ano para calcular os feriados.
///
/// ### Retorno
/// * `Result<Vec<Holiday>, UnexpectedError>`
///
/// # Example
///  ```
/// use brasilapi::holidays;
/// use brasilapi::holidays::Holiday;
///
/// let holidays:Vec<Holiday> = holidays::get_holidays("2022").await.unwrap();
/// ```
pub async fn get_holidays(year: &str) -> Result<Vec<Holiday>, UnexpectedError> {
    let response = HolidayService::get_holiday_request(year).await.unwrap();

    let status = response.status().as_u16();

    if status != 200 {
        let error: Error = serde_json::from_str(&response.text().await.unwrap()).unwrap();

        return Err(UnexpectedError {
            code: status,
            message: error.clone().message,
            error: Errored::NotFound(error),
        });
    }

    let holidays: Vec<Holiday> = serde_json::from_str(&response.text().await.unwrap()).unwrap();

    Ok(holidays)
}
