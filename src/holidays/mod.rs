use crate::{error::*, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Holiday {
    pub date: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub full_name: Option<String>,
}

pub struct HolidayService {
    base_url: String,
}

impl HolidayService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get_holiday_request(&self, year: &str) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/feriados/v1/{}", self.base_url, year);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }
}

/// #### `get_holidays(year: &str)`
/// Lista os feriados nacionais de determinado ano.
///
/// ### Argumento
/// * `year:&str` => Ano para calcular os feriados.
///
/// ### Retorno
/// * `Result<Vec<Holiday>, Error>`
///
/// # Exemplo
///  ```
/// use brasilapi::holidays;
/// use brasilapi::holidays::Holiday;
///
/// #[tokio::main]
/// async fn main() {
///     let holidays:Vec<Holiday> = holidays::get_holidays("2022").await.unwrap();
/// }
/// ```
pub async fn get_holidays(year: &str) -> Result<Vec<Holiday>, Error> {
    let holiday_service = HolidayService::new(BRASIL_API_URL);

    let response = holiday_service.get_holiday_request(year).await?;

    let body = response.text().await.unwrap();
    let holidays: Vec<Holiday> = serde_json::from_str(&body).unwrap();

    Ok(holidays)
}

/// #### `get_holiday(year: &str, month: &str, day: &str)`
/// Retorna um feriado a partir de uma data.
///
/// ### Argumento
/// * `year:&str`   => Ano do feriado.
/// * `month:&str`  => Mês do feriado.
/// * `day:&str`    => Dia do feriado.
///
/// ### Retorno
/// * `Result<Holiday, Error>`
///
/// # Example
/// ```
/// use brasilapi::holidays;
/// use brasilapi::holidays::Holiday;
///
/// #[tokio::main]
/// async fn main() {
///     let holiday:Holiday = holidays::get_holiday("2022", "09", "07").await.unwrap();
/// }
/// ```
pub async fn get_holiday(year: &str, month: &str, day: &str) -> Result<Holiday, Error> {
    let holidays = get_holidays(year).await?;

    let holiday_position = holidays
        .iter()
        .position(|holiday| holiday.date == format!("{year}-{month}-{day}"));

    match holiday_position {
        Some(position) => Ok(holidays.get(position).unwrap().clone()),
        None => Err(Error::new(
            String::from("holiday not found"),
            Errored::NotFound,
            Some(404),
        )),
    }
}

#[cfg(test)]
mod holidays_tests {
    use super::*;

    #[tokio::test]
    async fn get_holidays_test() {
        let holidays = get_holidays("2022").await.unwrap();

        let holyday = get_holiday("2022", "01", "01").await.unwrap();

        assert!(holidays.contains(&holyday));
    }

    #[tokio::test]
    async fn get_holidays_error_test() {
        let holidays = get_holidays("2").await;

        assert!(holidays.is_err());
    }

    #[tokio::test]
    async fn get_holiday_test() {
        let holiday = get_holiday("2022", "09", "07").await.unwrap();

        assert_eq!(holiday.name, "Independência do Brasil");
    }

    #[tokio::test]
    async fn get_holiday_error_test() {
        let holiday = get_holiday("2022", "10", "02").await;

        assert!(holiday.is_err());
    }
}
