use crate::{error::*, spec::BRASIL_API_URL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cnpj {
    cnpj: Option<String>,
    identificador_matriz_filial: Option<i32>,
    descricao_matriz_filial: Option<String>,
    razao_social: Option<String>,
    nome_fantasia: Option<String>,
    situacao_cadastral: Option<i32>,
    descricao_situacao_cadastral: Option<String>,
    data_situacao_cadastral: Option<String>,
    motivo_situacao_cadastral: Option<i32>,
    nome_cidade_exterior: Option<String>,
    codigo_natureza_juridica: Option<i32>,
    data_inicio_atividade: Option<String>,
    cnae_fiscal: Option<i32>,
    cnae_fiscal_descricao: Option<String>,
    descricao_tipo_logradouro: Option<String>,
    logradouro: Option<String>,
    numero: Option<String>,
    complemento: Option<String>,
    bairro: Option<String>,
    cep: Option<String>,
    uf: Option<String>,
    codigo_municipio: Option<i32>,
    municipio: Option<String>,
    ddd_telefone_1: Option<String>,
    ddd_telefone_2: Option<String>,
    ddd_fax: Option<String>,
    qualificacao_do_responsavel: Option<i32>,
    capital_social: Option<i64>,
    porte: Option<String>,
    descricao_porte: Option<String>,
    opcao_pelo_simples: Option<bool>,
    data_opcao_pelo_simples: Option<String>,
    data_exclusao_do_simples: Option<String>,
    opcao_pelo_mei: Option<bool>,
    situacao_especial: Option<String>,
    data_situacao_especial: Option<String>,
    cnaes_secundarias: Option<Vec<Cnaes>>,
    qsa: Option<Vec<Qsa>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cnaes {
    codigo: Option<i32>,
    descricao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Qsa {
    identificador_de_socio: Option<i32>,
    nome_socio: Option<String>,
    cnpj_cpf_do_socio: Option<String>,
    codigo_qualificacao_socio: Option<i32>,
    percentual_capital_social: Option<i32>,
    data_entrada_sociedade: Option<String>,
    cpf_representante_legal: Option<String>,
    nome_representante_legal: Option<String>,
    codigo_qualificacao_representante_legal: Option<i32>,
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
