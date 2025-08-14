use crate::spec::BRASIL_API_URL;
use reqwest::Response;
use serde::Deserialize;

#[derive(Debug)]
enum TypeFecth {
    All,
    Code,
    Description,
    None,
}

#[derive(Debug)]
pub struct NcmService {
    base_url: String,
    type_fecth: TypeFecth,
    target_ncm_code: Option<String>,
    target_ncm_description: Option<String>,
}

impl NcmService {
    pub fn get() -> Self {
        Self {
            base_url: BRASIL_API_URL.to_string(),
            type_fecth: TypeFecth::None,
            target_ncm_code: None,
            target_ncm_description: None,
        }
    }

    pub async fn all(&mut self) -> Vec<Ncm> {
        self.type_fecth = TypeFecth::All;
        let response: Response = self.conn_with_api().await;
        let body: String = response.text().await.unwrap();
        let vec_ncm: Vec<Ncm> = serde_json::from_str(&body).unwrap();
        vec_ncm
    }

    pub async fn code(&mut self, code: &str) -> Ncm {
        self.type_fecth = TypeFecth::Code;
        self.target_ncm_code = Some(code.to_string());
        let response: Response = self.conn_with_api().await;
        let body: String = response.text().await.unwrap();
        let ncm: Ncm = serde_json::from_str(&body).expect("Error");
        ncm
    }

    pub async fn description(&mut self, description: &str) -> Vec<Ncm> {
        self.type_fecth = TypeFecth::Description;
        self.target_ncm_description = Some(description.to_string());
        let response: Response = self.conn_with_api().await;
        let body: String = response.text().await.unwrap();
        let ncm: Vec<Ncm> = serde_json::from_str(&body).unwrap();
        ncm
    }

    async fn conn_with_api(&self) -> Response {
        let response: Response = reqwest::get(&self.set_url_by_type_fetch())
            .await
            .expect("Error:");
        response
    }

    fn set_url_by_type_fetch(&self) -> String {
        match self.type_fecth {
            TypeFecth::All => format!("{}/api/ncm/v1", self.base_url),
            TypeFecth::Code => format!(
                "{}/api/ncm/v1/{}",
                self.base_url,
                self.target_ncm_code.as_ref().unwrap()
            ),
            TypeFecth::Description => format!(
                "{}/api/ncm/v1?search={}",
                self.base_url,
                self.target_ncm_description.as_ref().unwrap()
            ),
            TypeFecth::None => panic!("Deu merda"),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Ncm {
    codigo: String,
    descricao: String,
    data_inicio: String,
    data_fim: String,
}

#[cfg(test)]
mod ncm_test {
    use crate::{
        ncm::{NcmService, TypeFecth},
        spec::BRASIL_API_URL,
    };

    const NCM_CODE: &'static str = "33051000";
    const NCM_DESCRIPTION: &'static str = "carne";

    #[test]
    fn set_url_for_while_type_fetch_is_all() {
        let ncm_service_set_all: NcmService = StateNcmService::All.set();
        assert_eq!(
            ncm_service_set_all.set_url_by_type_fetch(),
            PossibleUrl::All.get()
        );
    }

    #[test]
    fn set_url_for_while_type_fetch_is_code() {
        let ncm_service_set_code: NcmService = StateNcmService::Code.set();
        assert_eq!(
            ncm_service_set_code.set_url_by_type_fetch(),
            PossibleUrl::Code.get()
        );
    }

    #[test]
    fn set_url_for_while_type_fetch_is_description() {
        let ncm_service_set_description: NcmService = StateNcmService::Description.set();
        assert_eq!(
            ncm_service_set_description.set_url_by_type_fetch(),
            PossibleUrl::Description.get()
        );
    }

    #[tokio::test]
    async fn status_200_with_api_all_ncm() {
        let ncm_service_set_all: NcmService = StateNcmService::All.set();
        assert_eq!(ncm_service_set_all.conn_with_api().await.status(), 200);
    }

    #[tokio::test]
    async fn status_200_with_api_code_ncm() {
        let ncm_service_set_code: NcmService = StateNcmService::Code.set();
        assert_eq!(ncm_service_set_code.conn_with_api().await.status(), 200);
    }

    #[tokio::test]
    async fn status_200_with_api_description_ncm() {
        let ncm_service_set_description: NcmService = StateNcmService::Description.set();
        assert_eq!(
            ncm_service_set_description.conn_with_api().await.status(),
            200
        );
    }

    #[tokio::test]
    async fn mount_vec_with_fetch_all_ncm() {
        NcmService::get().all().await;
    }

    #[tokio::test]
    async fn mount_ncm_with_fetch_code() {
        NcmService::get().code(NCM_CODE).await;
    }

    #[tokio::test]
    async fn mount_struct_ncm_with_fetch_description() {
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
                    type_fecth: TypeFecth::All,
                    target_ncm_code: None,
                    target_ncm_description: None,
                },
                Self::Code => NcmService {
                    base_url: BRASIL_API_URL.to_string(),
                    type_fecth: TypeFecth::Code,
                    target_ncm_code: Some(NCM_CODE.to_string()),
                    target_ncm_description: None,
                },
                Self::Description => NcmService {
                    base_url: BRASIL_API_URL.to_string(),
                    type_fecth: TypeFecth::Description,
                    target_ncm_code: None,
                    target_ncm_description: Some(NCM_DESCRIPTION.to_string()),
                },
            }
        }
    }
}
