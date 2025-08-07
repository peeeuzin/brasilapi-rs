use crate::{error::Error, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Isbn {
    pub isbn: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub authors: Option<Vec<String>>,
    pub publisher: Option<String>,
    pub synopsis: Option<String>,
    pub dimensions: Option<Dimensions>,
    pub year: Option<u32>,
    pub format: Option<Format>,
    pub page_count: Option<u32>,
    pub subjects: Option<Vec<String>>,
    pub location: Option<String>,
    pub retail_price: Option<RetailPrice>,
    pub cover_url: Option<String>,
    pub provider: Provider,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Unit {
    Centimeter,
    Inch,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Dimensions {
    pub width: f64,
    pub height: f64,
    pub unit: Unit,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Format {
    Physical,
    Digital,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RetailPrice {
    pub currency: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Provider {
    Cbl,
    MercadoEditorial,
    OpenLibrary,
    GoogleBooks,
}

pub struct IsbnService {
    base_url: String,
}

impl IsbnService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get_isbn_request(&self, isbn_code: &str) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/isbn/v1/{}", self.base_url, isbn_code);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }

    async fn validate_isbn(&self, isbn_code: &str) -> Result<bool, Error> {
        let response = self.get_isbn_request(isbn_code).await;

        match response {
            Ok(_) => Ok(true),
            Err(e) => match e.code {
                Some(404) => Ok(false),
                _ => Err(e),
            },
        }
    }
}

/// #### `get_isbn(isbn_code: &str)`
/// Busca por **ISBN** nos provedores:
/// * CBL;
/// * Mercado Editorial;
/// * Open Library;
/// * Google Books.
///
/// ### Argumento
/// * `isbn_code: &str` => ISBN para consulta.
///
/// ### Retorno
/// * `Result<Isbn, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::isbn;
///
/// #[tokio::main]
/// async fn main() {
///    let isbn = isbn::get_isbn("8535914846").await.unwrap();  
/// }
/// ```
pub async fn get_isbn(isbn_code: &str) -> Result<Isbn, Error> {
    let isbn_service = IsbnService::new(BRASIL_API_URL);

    let response = isbn_service.get_isbn_request(isbn_code).await?;

    let body = response.text().await.unwrap();
    let isbn: Isbn = serde_json::from_str(&body).unwrap();

    Ok(isbn)
}

/// #### `validate(isbn_code: &str)`
/// Valida um ISBN.
///
/// ### Argumento
/// * `isbn_code: &str` => ISBN a ser validado.
///
/// ### Retorno
/// * `Result<bool, Error>`
///
/// # Exemplo
/// ```
/// use brasilapi::cep;
///
/// #[tokio::main]
/// async fn main() {
///   let is_valid = isbn::validate("8535914846").await.unwrap();  
/// }
pub async fn validate(isbn_code: &str) -> Result<bool, Error> {
    IsbnService::new(BRASIL_API_URL)
        .validate_isbn(isbn_code)
        .await
}

#[cfg(test)]
mod isbn_tests {
    use super::*;

    #[tokio::test]
    async fn get_isbn_test() {
        let isbn = get_isbn("8575228609").await.unwrap();

        assert_eq!(isbn.title, "Programação em Rust 2ª Edição");
        assert_eq!(
            isbn.subtitle,
            Some("Desenvolvimento de sistemas rápidos e seguros".to_string())
        );
        assert_eq!(
            isbn.authors,
            Some(
                [
                    "Jim Blandy",
                    "Leonora F. S. Tindall",
                    "Jason Orendorff",
                    "Rubens Prates",
                    "Edson Furmankiewicz",
                ]
                .iter()
                .map(|s| s.to_string())
                .collect()
            )
        );
        assert_eq!(isbn.publisher, Some("Novatec Editora".to_string()));
        assert_eq!(
            isbn.dimensions,
            Some(Dimensions {
                width: 17.2,
                height: 23.5,
                unit: Unit::Centimeter,
            })
        );
        assert_eq!(isbn.year, Some(2023));
        assert_eq!(isbn.format, Some(Format::Physical));
        assert_eq!(isbn.page_count, Some(800));
        assert_eq!(
            isbn.subjects,
            Some(
                ["Tecnologia (ciências aplicadas)", "Rust"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
        assert_eq!(isbn.location, Some("São Paulo, SP".to_string()));
        assert_eq!(isbn.provider, Provider::Cbl);
    }

    #[tokio::test]
    async fn get_isbn_error() {
        let isbn = get_isbn("1234567890").await;

        assert!(isbn.is_err());
    }

    #[tokio::test]
    async fn validate_test() {
        let isbn = validate("8575228609").await.unwrap();

        assert!(isbn);
    }
}
