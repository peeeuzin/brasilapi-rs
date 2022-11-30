use crate::{
    error::{Error, Errored, UnexpectedError},
    spec::BRASIL_API_URL,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

/// ## `get_holiday(year: &str, month: &str, day: &str)`
/// Retorna um feriado a partir de uma data.
///
/// ### Argumento
/// * `year:&str`   => Ano do feriado.
/// * `month:&str`  => Mês do feriado.
/// * `day:&str`    => Dia do feriado.
///
/// ### Retorno
/// * `Result<Holiday, UnexpectedError>`
///
/// # Example
/// ```
/// use brasilapi::holidays;
/// use brasilapi::holidays::Holiday;
///
/// let holiday:Holiday = holidays::get_holiday("2022", "09", "07").await.unwrap();
/// ```
pub async fn get_holiday(year: &str, month: &str, day: &str) -> Result<Holiday, UnexpectedError> {
    let response = get_holidays(year).await;

    match response {
        Ok(holidays) => {
            let holiday_position = holidays
                .iter()
                .position(|holiday| holiday.date == format!("{year}-{month}-{day}"));

            match holiday_position {
                Some(position) => {
                    return Ok(holidays.get(position).unwrap().clone());
                }
                None => {
                    return Err(UnexpectedError {
                        code: 404,
                        message: String::from("holiday not found"),
                        error: Errored::Unexpected,
                    });
                }
            }
        }
        Err(error) => {
            return Err(error);
        }
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
