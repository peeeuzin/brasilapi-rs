use crate::{error::*, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cnpj {
    pub cnpj: Option<String>,
    pub identificador_matriz_filial: Option<i32>,
    pub descricao_matriz_filial: Option<String>,
    pub razao_social: Option<String>,
    pub nome_fantasia: Option<String>,
    pub situacao_cadastral: Option<i32>,
    pub descricao_situacao_cadastral: Option<String>,
    pub data_situacao_cadastral: Option<String>,
    pub motivo_situacao_cadastral: Option<i32>,
    pub nome_cidade_exterior: Option<String>,
    pub codigo_natureza_juridica: Option<i32>,
    pub data_inicio_atividade: Option<String>,
    pub cnae_fiscal: Option<i32>,
    pub cnae_fiscal_descricao: Option<String>,
    pub descricao_tipo_logradouro: Option<String>,
    pub logradouro: Option<String>,
    pub numero: Option<String>,
    pub complemento: Option<String>,
    pub bairro: Option<String>,
    pub cep: Option<String>,
    pub uf: Option<String>,
    pub codigo_municipio: Option<i32>,
    pub municipio: Option<String>,
    pub ddd_telefone_1: Option<String>,
    pub ddd_telefone_2: Option<String>,
    pub ddd_fax: Option<String>,
    pub qualificacao_do_responsavel: Option<i32>,
    pub capital_social: Option<i64>,
    pub porte: Option<String>,
    pub descricao_porte: Option<String>,
    pub opcao_pelo_simples: Option<bool>,
    pub data_opcao_pelo_simples: Option<String>,
    pub data_exclusao_do_simples: Option<String>,
    pub opcao_pelo_mei: Option<bool>,
    pub situacao_especial: Option<String>,
    pub data_situacao_especial: Option<String>,
    pub cnaes_secundarias: Option<Vec<Cnaes>>,
    pub qsa: Option<Vec<Qsa>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cnaes {
    pub codigo: Option<i32>,
    pub descricao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Qsa {
    pub identificador_de_socio: Option<i32>,
    pub nome_socio: Option<String>,
    pub cnpj_cpf_do_socio: Option<String>,
    pub codigo_qualificacao_socio: Option<i32>,
    pub percentual_capital_social: Option<i32>,
    pub data_entrada_sociedade: Option<String>,
    pub cpf_representante_legal: Option<String>,
    pub nome_representante_legal: Option<String>,
    pub codigo_qualificacao_representante_legal: Option<i32>,
}

pub struct CnpjService {
    base_url: String,
}

impl CnpjService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get_cnpj_request(&self, cnpj_code: &str) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/cnpj/v1/{}", self.base_url, cnpj_code);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }
}

/// #### `get_cnpj(cnpj: &str)`
/// Busca por CNPJ na API Minha Receita.
///
/// ### Argumento
/// * `cnpj:&str` => CNPJ para consulta.
///
/// ### Retorno
/// * `Result<Cnpj, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::cnpj;
///
/// #[tokio::main]
/// async fn main() {
///    let cnpj = cnpj::get_cnpj("00000000000191").await.unwrap();  
/// }
/// ```
pub async fn get_cnpj(cnpj: &str) -> Result<Cnpj, Error> {
    let cnpj_service = CnpjService::new(BRASIL_API_URL);

    let response = cnpj_service.get_cnpj_request(cnpj).await?;

    let body = response.text().await.unwrap();
    let cnpj: Cnpj = serde_json::from_str(&body).unwrap();

    Ok(cnpj)
}

#[cfg(test)]
mod cnpj_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_cnpj() {
        let cnpj = get_cnpj("00000000000191").await.unwrap();

        assert_eq!(cnpj.cnpj, Some("00000000000191".to_string()));
    }
}
