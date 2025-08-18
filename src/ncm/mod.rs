use crate::{error::*, spec::BRASIL_API_URL};
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Ncm {
    pub codigo: Option<String>,
    pub descricao: Option<String>,
    pub data_inicio: Option<String>,
    pub data_fim: Option<String>,
    pub tipo_ato: Option<String>,
    pub numero_ato: Option<String>,
    pub ano_ato: Option<String>,
}

pub enum Fetch {
    All,
    Code(String),
    Description(String),
}

pub struct NcmService {
    base_url: String,
    fetch: Fetch,
}

impl NcmService {
    pub fn new(base_url: &str, fetch: Fetch) -> Self {
        match &fetch {
            Fetch::All => (),
            Fetch::Code(c) => Self::validade_code(&c),
            Fetch::Description(_) => (),
        }

        Self {
            base_url: base_url.to_string(),
            fetch: fetch,
        }
    }

    async fn get_ncm_request(&self) -> Result<reqwest::Response, Error> {
        let url = self.set_url_by_fetch();

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }

    fn validade_code(code: &String) {
        let code_without_dot: String = Self::take_dot(code);

        code_without_dot
            .parse::<i32>()
            .expect("Ncm's code can't contain letters");
    }

    fn set_url_by_fetch(&self) -> String {
        match self.fetch {
            Fetch::All => format!("{}/api/ncm/v1", self.base_url),
            Fetch::Code(ref c) => format!("{}/api/ncm/v1/{}", self.base_url, Self::take_dot(&c)),
            Fetch::Description(ref d) => format!("{}/api/ncm/v1?search={}", self.base_url, d),
        }
    }

    fn take_dot(code: &String) -> String {
        let code_without_dot: String = code.replace(".", "");

        code_without_dot
    }
}

/// #### `get_ncm_all()`
/// Busca todas as Nomenclatura Comum do Mercosul(NCM) na API Minha Receita.
///
/// ### Retorno
/// * `Result<Vec<Ncm>, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::ncm;
///
/// #[tokio::main]
/// async fn main() {
///     let ncms = ncm::get_ncm_all().await.unwrap();
/// }
/// ```

/// #### `get_ncm_by_code(code: &str)`
/// Busca por Nomenclatura Comum do Mercosul(NCM) na API Minha Receita que corresponde
/// com o código.
///
/// ### Retorno
/// * `Result<Ncm, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::ncm;
///
/// #[tokio::main]
/// async fn main() {
///     let ncm = ncm::get_ncm_by_code(&"3305.10.00").await.unwrap();
/// }
/// ```
///

/// #### `get_ncm_by_description(code: &str)`
/// Busca todas as Nomenclatura Comum do Mercosul(NCM) na API Minha Receita que
/// corresponde com a descrição.
///
/// ### Retorno
/// * `Result<Vec<Ncm >, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::ncm;
///
/// #[tokio::main]
/// async fn main() {
///     let ncms = ncm::get_ncm_by_description(&"tijolo").await.unwrap();
/// }
/// ```
///

pub async fn get_ncm_all() -> Result<Vec<Ncm>, Error> {
    let ncm_service: NcmService = NcmService::new(BRASIL_API_URL, Fetch::All);

    let response: Response = ncm_service.get_ncm_request().await?;

    let body: String = response.text().await.unwrap();

    let ncms: Vec<Ncm> = serde_json::from_str(&body).unwrap();

    Ok(ncms)
}

pub async fn get_ncm_by_code(code: &str) -> Result<Ncm, Error> {
    let ncm_service: NcmService = NcmService::new(BRASIL_API_URL, Fetch::Code(code.to_string()));

    let response: Response = ncm_service.get_ncm_request().await?;

    let body: String = response.text().await.unwrap();

    let ncm: Ncm = serde_json::from_str(&body).unwrap();

    Ok(ncm)
}

pub async fn get_ncm_by_description(description: &str) -> Result<Vec<Ncm>, Error> {
    let ncm_service: NcmService =
        NcmService::new(BRASIL_API_URL, Fetch::Description(description.to_string()));

    let response: Response = ncm_service.get_ncm_request().await?;

    let body: String = response.text().await.unwrap();

    let ncms: Vec<Ncm> = serde_json::from_str(&body).unwrap();

    Ok(ncms)
}

#[cfg(test)]
mod ncm_tests {
    use super::*;

    #[test]
    fn code_to_fetch_cant_contain_dot_in_ulr() {
        let code: String = "33051000".to_string();
        let expected_code: String = NcmService::take_dot(&"3305.10.00".to_string());

        assert_eq!(code, expected_code);
    }

    #[test]
    #[should_panic(expected = "Ncm's code can't contain letters")]
    fn code_to_fetch_cant_contain_letters() {
        NcmService::new(BRASIL_API_URL, Fetch::Code("carne".to_string()));
    }

    #[test]
    fn set_url_by_fetch_when_is_all() {
        let ncm_service: NcmService = NcmService::new(BRASIL_API_URL, Fetch::All);

        let url: String = ncm_service.set_url_by_fetch();
        let expected_url: String = "https://brasilapi.com.br/api/ncm/v1".to_string();

        assert_eq!(url, expected_url);
    }

    #[test]
    fn set_url_by_fetch_when_is_code() {
        let ncm_service: NcmService =
            NcmService::new(BRASIL_API_URL, Fetch::Code("3305.10.00".to_string()));

        let url: String = ncm_service.set_url_by_fetch();
        let expected_url: String = "https://brasilapi.com.br/api/ncm/v1/33051000".to_string();

        assert_eq!(url, expected_url);
    }

    #[test]
    fn set_url_by_fetch_when_is_description() {
        let ncm_service: NcmService =
            NcmService::new(BRASIL_API_URL, Fetch::Description("carne".to_string()));

        let url: String = ncm_service.set_url_by_fetch();
        let expected_url: String = "https://brasilapi.com.br/api/ncm/v1?search=carne".to_string();

        assert_eq!(url, expected_url);
    }

    #[tokio::test]
    async fn get_ncm_request_when_fetch_is_all() {
        NcmService::new(BRASIL_API_URL, Fetch::All)
            .get_ncm_request()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_ncm_request_when_fetch_is_code() {
        NcmService::new(BRASIL_API_URL, Fetch::Code("3305.10.00".to_string()))
            .get_ncm_request()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_ncm_request_when_fetch_is_description() {
        NcmService::new(BRASIL_API_URL, Fetch::Description("carne".to_string()))
            .get_ncm_request()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn mount_ncm_when_fetch_is_all() {
        get_ncm_all().await.unwrap();
    }

    #[tokio::test]
    async fn mount_ncm_when_fetch_is_code() {
        let ncm: Ncm = get_ncm_by_code(&"3305.10.00").await.unwrap();

        let expected_ncm: Ncm = Ncm {
            codigo: Some("3305.10.00".to_string()),
            descricao: Some("- Xampus".to_string()),
            data_inicio: Some("2022-04-01".to_string()),
            data_fim: Some("9999-12-31".to_string()),
            tipo_ato: Some("Res Camex".to_string()),
            numero_ato: Some("272".to_string()),
            ano_ato: Some("2021".to_string()),
        };

        assert_eq!(ncm, expected_ncm);
    }

    #[tokio::test]
    #[should_panic]
    async fn error_mount_ncm_when_dont_exist_code() {
        get_ncm_by_code(&"0000000000").await.unwrap();
    }
}
