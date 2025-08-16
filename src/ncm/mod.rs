use crate::error::Error;
use crate::spec::BRASIL_API_URL;
use std::collections::HashMap;

pub type Ncm = HashMap<String, String>;

#[derive(Debug)]
enum Fetch {
    All,
    Code(String),
    Description(String),
    None,
}

#[derive(Debug)]
pub struct NcmService {
    base_url: String,
    fetch: Fetch,
}

impl NcmService {
    pub fn get() -> Self {
        Self {
            base_url: BRASIL_API_URL.to_string(),
            fetch: Fetch::None,
        }
    }

    pub async fn all(&mut self) -> Result<Vec<Ncm>, Error> {
        let result_fetch: String = self.fetch(Fetch::All).await.unwrap();
        let vec_ncm: Vec<Ncm> = self.assembly_ncm_hash_map(result_fetch);
        Ok(vec_ncm)
    }

    pub async fn code(&mut self, code: &str) -> Result<Vec<Ncm>, Error> {
        let result_fetch: String = self.fetch(Fetch::All).await?;
        let vec_ncm: Vec<Ncm> = self.assembly_ncm_hash_map(result_fetch);
        Ok(vec_ncm)
    }

    pub async fn code(&mut self, code: &str) -> Result<Vec<Ncm>, Error> {
        let result_fetch: String = self.fetch(Fetch::Code(code.to_string())).await?;
        let vec_ncm: Vec<Ncm> = self.assembly_ncm_hash_map(result_fetch);
        Ok(vec_ncm)
    }

    pub async fn description(&mut self, description: &str) -> Result<Vec<Ncm>, Error> {
        let result_fetch: String = self
            .fetch(Fetch::Description(description.to_string()))
            .await
            .await?;
        let vec_ncm: Vec<Ncm> = self.assembly_ncm_hash_map(result_fetch);
        Ok(vec_ncm)
    }

    async fn fetch(&mut self, fetch: Fetch) -> Result<String, Error> {
        self.set_environment_for_fetch(fetch);
        let fetch_result: Result<reqwest::Response, reqwest::Error> = self.request_for_api().await;
        match fetch_result {
            Ok(r) => Ok(r.text().await.unwrap()),
            Err(e) => Err(Error::from_error(e)),
        }
    }

    fn assembly_ncm_hash_map(&self, result: String) -> Vec<Ncm> {
        match self.fetch {
            Fetch::Code(_) => vec![serde_json::from_str(&result).unwrap()],
            Fetch::Description(_) => serde_json::from_str(&result).unwrap(),
            Fetch::All => serde_json::from_str(&result).unwrap(),
            Fetch::None => panic!("Don't use Fetch::None in assembly_ncm_hash_map"),
        }
    }

    fn set_environment_for_fetch(&mut self, fetch: Fetch) {
        self.fetch = fetch;
    }

    async fn request_for_api(&self) -> Result<reqwest::Response, reqwest::Error> {
        let url: String = self.set_url_by_fetch();
        let response: Result<reqwest::Response, reqwest::Error> = reqwest::get(&url).await;
        response
    }

    fn set_url_by_fetch(&self) -> String {
        match self.fetch {
            Fetch::All => format!("{}/api/ncm/v1", self.base_url),
            Fetch::Code(ref c) => format!("{}/api/ncm/v1/{}", self.base_url, c),
            Fetch::Description(ref d) => format!("{}/api/ncm/v1?search={}", self.base_url, d),
            Fetch::None => panic!("Don't use Fetch::None in set_url_by_fetch"),
        }
    }
}

#[cfg(test)]
mod ncm_test {
    use crate::{
        ncm::{Fetch, NcmService},
        spec::BRASIL_API_URL,
    };

    const NCM_CODE: &'static str = "33051000";
    const NCM_DESCRIPTION: &'static str = "carne";

    macro_rules! set_fetch_ncm_service {
        ($type:expr) => {
            NcmService {
                base_url: BRASIL_API_URL.to_string(),
                fetch: $type,
            }
        };
    }

    macro_rules! expected_url {
        ($fetch:expr) => {
            match $fetch {
                Fetch::All => "https://brasilapi.com.br/api/ncm/v1".to_string(),
                Fetch::Code(c) => format!("https://brasilapi.com.br/api/ncm/v1/{}", c),
                Fetch::Description(d) => {
                    format!("https://brasilapi.com.br/api/ncm/v1?search={}", d)
                }
                Fetch::None => panic!("Don't use Fetch::None in set_url_by_fetch"),
            }
        };
    }

    #[test]
    fn set_url_for_while_fetch_is_all() {
        let ncm_service: NcmService = set_fetch_ncm_service!(Fetch::All);

        assert_eq!(ncm_service.set_url_by_fetch(), expected_url!(Fetch::All));
    }

    #[test]
    fn set_url_for_while_fetch_is_code() {
        let ncm_service: NcmService = set_fetch_ncm_service!(Fetch::Code(NCM_CODE.to_string()));

        assert_eq!(
            ncm_service.set_url_by_fetch(),
            expected_url!(Fetch::Code(NCM_CODE.to_string()))
        )
    }

    #[test]
    fn set_url_for_while_fetch_is_description() {
        let ncm_service: NcmService =
            set_fetch_ncm_service!(Fetch::Description(NCM_DESCRIPTION.to_string()));

        assert_eq!(
            ncm_service.set_url_by_fetch(),
            expected_url!(Fetch::Description(NCM_DESCRIPTION.to_string()))
        );
    }

    #[test]
    #[should_panic(expected = "Don't use Fetch::None in set_url_by_fetch")]
    fn panic_set_url_when_fetch_is_none() {
        let ncm_service: NcmService = set_fetch_ncm_service!(Fetch::None);
        ncm_service.set_url_by_fetch();
    }

    #[tokio::test]
    async fn status_200_with_api_when_fetch_is_all() {
        let ncm_service: NcmService = set_fetch_ncm_service!(Fetch::All);

        assert_eq!(ncm_service.request_for_api().await.unwrap().status(), 200);
    }

    #[tokio::test]
    async fn status_200_with_api_when_fetch_is_code() {
        let ncm_service: NcmService = set_fetch_ncm_service!(Fetch::Code(NCM_CODE.to_string()));

        assert_eq!(ncm_service.request_for_api().await.unwrap().status(), 200);
    }

    #[tokio::test]
    async fn status_200_with_api_when_fetch_is_description() {
        let ncm_service: NcmService =
            set_fetch_ncm_service!(Fetch::Description(NCM_DESCRIPTION.to_string()));

        assert_eq!(ncm_service.request_for_api().await.unwrap().status(), 200);
    }

    #[tokio::test]
    async fn assembly_ncm_hash_map_when_fetch_is_all() {
        NcmService::get().all().await.unwrap();
    }

    #[tokio::test]
    async fn assembly_ncm_hash_map_when_fetch_is_code() {
        NcmService::get().code(NCM_CODE).await.unwrap();
    }

    #[tokio::test]
    async fn assembly_ncm_hash_map_when_fetch_is_description() {
        NcmService::get()
            .description(NCM_DESCRIPTION)
            .await
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "Don't use Fetch::None in assembly_ncm_hash_map")]
    fn assembly_ncm_hash_map_when_fetch_is_none() {
        let ncm_service: NcmService = set_fetch_ncm_service!(Fetch::None);
        ncm_service.assembly_ncm_hash_map("".to_string());
    }
}