use std::collections::HashMap;

use crate::spec::BRASIL_API_URL;
use reqwest::Response;

type NcmCode = str;
type NcmDescription = str;
type Ncm = HashMap<String, String>;

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

    pub async fn all(&mut self) -> Vec<Ncm> {
        let result_fetch: String = self.fetch(Fetch::All).await;
        let vec_ncm: Vec<Ncm> = self.assembly_ncm_hash_map(result_fetch);
        vec_ncm
    }

    pub async fn code(&mut self, code: &NcmCode) -> Ncm {
        let result_fetch: String = self.fetch(Fetch::Code(code.to_string())).await;
        let mut vec_ncm: Vec<Ncm> = self.assembly_ncm_hash_map(result_fetch);
        let ncm: Ncm = self.validade_ncm_to_code_fetch(&mut vec_ncm);
        ncm
    }

    pub async fn description(&mut self, description: &NcmDescription) -> Vec<Ncm> {
        let result_fetch: String = self
            .fetch(Fetch::Description(description.to_string()))
            .await;
        let vec_ncm: Vec<Ncm> = self.assembly_ncm_hash_map(result_fetch);
        vec_ncm
    }

    async fn fetch(&mut self, fetch: Fetch) -> String {
        self.set_environment_for_fetch(fetch);
        let fetch_result: Response = self.request_for_api().await;
        fetch_result.text().await.unwrap()
    }

    fn assembly_ncm_hash_map(&self, result: String) -> Vec<Ncm> {
        match self.fetch {
            Fetch::Code(_) => vec![serde_json::from_str(&result).unwrap()],
            Fetch::Description(_) => serde_json::from_str(&result).unwrap(),
            Fetch::All => serde_json::from_str(&result).unwrap(),
            _ => panic!("Deu merda"),
        }
    }

    fn validade_ncm_to_code_fetch(&self, vec_ncm: &mut Vec<Ncm>) -> Ncm {
        match vec_ncm.len() {
            1 => vec_ncm.remove(0),
            _ => panic!("Deu marda"),
        }
    }

    fn set_environment_for_fetch(&mut self, fetch: Fetch) {
        self.fetch = fetch;
    }

    async fn request_for_api(&self) -> Response {
        let url: String = self.set_url_by_fetch();
        let response: Response = reqwest::get(&url).await.expect("Error:");
        response
    }

    fn set_url_by_fetch(&self) -> String {
        match self.fetch {
            Fetch::All => format!("{}/api/ncm/v1", self.base_url),
            Fetch::Code(ref c) => format!("{}/api/ncm/v1/{}", self.base_url, c),
            Fetch::Description(ref d) => format!("{}/api/ncm/v1?search={}", self.base_url, d),
            Fetch::None => panic!("Deu merda"),
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

    #[test]
    fn set_url_for_while_fetch_is_all() {
        let ncm_service_set_all: NcmService = StateNcmService::All.set();
        assert_eq!(
            ncm_service_set_all.set_url_by_fetch(),
            PossibleUrl::All.get()
        );
    }

    #[test]
    fn set_url_for_while_fetch_is_code() {
        let ncm_service_set_code: NcmService = StateNcmService::Code.set();
        assert_eq!(
            ncm_service_set_code.set_url_by_fetch(),
            PossibleUrl::Code.get()
        );
    }

    #[test]
    fn set_url_for_while_fetch_is_description() {
        let ncm_service_set_description: NcmService = StateNcmService::Description.set();
        assert_eq!(
            ncm_service_set_description.set_url_by_fetch(),
            PossibleUrl::Description.get()
        );
    }

    #[tokio::test]
    async fn status_200_with_api_when_fetch_is_all() {
        let ncm_service_set_all: NcmService = StateNcmService::All.set();
        assert_eq!(ncm_service_set_all.request_for_api().await.status(), 200);
    }

    #[tokio::test]
    async fn status_200_with_api_when_fetch_is_code() {
        let ncm_service_set_code: NcmService = StateNcmService::Code.set();
        assert_eq!(ncm_service_set_code.request_for_api().await.status(), 200);
    }

    #[tokio::test]
    async fn status_200_with_api_when_fetch_is_description() {
        let ncm_service_set_description: NcmService = StateNcmService::Description.set();
        assert_eq!(
            ncm_service_set_description.request_for_api().await.status(),
            200
        );
    }

    #[tokio::test]
    async fn assembly_ncm_hash_map_when_fetch_is_all() {
        println!("{:?}", NcmService::get().all().await);
    }

    #[tokio::test]
    async fn assembly_ncm_hash_map_when_fetch_is_code() {
        NcmService::get().code(NCM_CODE).await;
    }

    #[tokio::test]
    async fn assembly_ncm_hash_map_when_fetch_is_description() {
        NcmService::get().description(NCM_DESCRIPTION).await;
    }

    enum PossibleUrl {
        All,
        Code,
        Description,
    }

    impl PossibleUrl {
        fn get(&self) -> String {
            match self {
                Self::All => "https://brasilapi.com.br/api/ncm/v1".to_string(),
                Self::Code => format!("https://brasilapi.com.br/api/ncm/v1/{}", NCM_CODE),
                Self::Description => format!(
                    "https://brasilapi.com.br/api/ncm/v1?search={}",
                    NCM_DESCRIPTION
                ),
            }
        }
    }

    enum StateNcmService {
        All,
        Code,
        Description,
    }

    impl StateNcmService {
        fn set(&self) -> NcmService {
            match self {
                Self::All => NcmService {
                    base_url: BRASIL_API_URL.to_string(),
                    fetch: Fetch::All,
                },
                Self::Code => NcmService {
                    base_url: BRASIL_API_URL.to_string(),
                    fetch: Fetch::Code(NCM_CODE.to_string()),
                },
                Self::Description => NcmService {
                    base_url: BRASIL_API_URL.to_string(),
                    fetch: Fetch::Description(NCM_DESCRIPTION.to_string()),
                },
            }
        }
    }
}
