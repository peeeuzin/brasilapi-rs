use serde::{Deserialize, Serialize};

use crate::{error::Error, spec::BRASIL_API_URL};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Brand {
    nome: String,
    valor: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Vehicle {
    valor: String,
    marca: String,
    modelo: String,
    #[serde(rename = "anoModelo")]
    ano_modelo: i64,
    combustivel: String,
    #[serde(rename = "codigoFipe")]
    codigo_fipe: String,
    #[serde(rename = "mesReferencia")]
    mes_referencia: String,
    #[serde(rename = "tipoVeiculo")]
    tipo_veiculo: i64,
    #[serde(rename = "siglaCombustivel")]
    sigla_combustivel: String,
    #[serde(rename = "dataConsulta")]
    data_consulta: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ReferenceTable {
    codigo: i32,
    mes: String,
}

pub enum VehicleType {
    Car,
    Motorcycle,
    Truck,
}

impl VehicleType {
    pub fn to_string(&self) -> &str {
        match self {
            VehicleType::Car => "carros",
            VehicleType::Motorcycle => "motos",
            VehicleType::Truck => "caminhoes",
        }
    }
}

pub struct FipeService {
    base_url: String,
}

impl FipeService {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    async fn get_brands_request(
        &self,
        vehicle_type: VehicleType,
        reference_table: Option<i64>,
    ) -> Result<reqwest::Response, Error> {
        let vehicle_type = vehicle_type.to_string();

        let reference_table = match reference_table {
            Some(reference_table) => format!("tabela_referencia={}", reference_table),
            None => "".to_string(),
        };

        let url = format!(
            "{}/api/fipe/marcas/v1/{}?{}",
            self.base_url, vehicle_type, reference_table
        );

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }

    async fn get_vehicle_request(
        &self,
        fipe_code: &str,
        reference_table: Option<i64>,
    ) -> Result<reqwest::Response, Error> {
        let reference_table = match reference_table {
            Some(reference_table) => format!("tabela_referencia={}", reference_table),
            None => "".to_string(),
        };

        let url = format!(
            "{}/api/fipe/preco/v1/{}?{}",
            self.base_url, fipe_code, reference_table
        );

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }

    async fn get_reference_tables_request(&self) -> Result<reqwest::Response, Error> {
        let url = format!("{}/api/fipe/tabelas/v1/", self.base_url);

        match reqwest::get(&url).await {
            Ok(response) => Error::from_response(response).await,
            Err(e) => Err(Error::from_error(e)),
        }
    }
}

/// #### `get_brands(vehicle_type: VehicleType, reference_table: Option<i64>)`
///
/// Lista as marcas de veículos referente ao tipo de veículo
///
/// ## Argumentos
/// * `vehicle_type: VehicleType` => Tipo de veículo para consulta.
/// * `reference_table: Option<i64>` => Tabela de referência para consulta.
///
/// ## Retorno
/// * `Result<Vec<Brand>, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::fipe;
///
/// #[tokio::main]
/// async fn main() {
///    let brands = fipe::get_brands(fipe::VehicleType::Car, None).await.unwrap();
/// }
/// ```
pub async fn get_brands(
    vehicle_type: VehicleType,
    reference_table: Option<i64>,
) -> Result<Vec<Brand>, Error> {
    let fipe_service = FipeService::new(BRASIL_API_URL);

    let response = fipe_service
        .get_brands_request(vehicle_type, reference_table)
        .await?;

    let body = response.text().await.unwrap();
    let brands: Vec<Brand> = serde_json::from_str(&body).unwrap();

    Ok(brands)
}

/// #### `get_vehicles(fipe_code: &str, reference_table: Option<i64>)`
/// Consulta o preço do veículo segundo a tabela fipe.
///
/// ## Argumentos
/// * `fipe_code: &str` => Código fipe do veículo.
/// * `reference_table: Option<i64>` => Tabela de referência para consulta.
///
/// ## Retorno
/// * `Result<Vec<Vehicle>, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::fipe;
///
/// #[tokio::main]
/// async fn main() {
///    let vehicles = fipe::get_vehicles("008274-0", None).await.unwrap();
/// }
/// ```
pub async fn get_vehicles(
    fipe_code: &str,
    reference_table: Option<i64>,
) -> Result<Vec<Vehicle>, Error> {
    let fipe_service = FipeService::new(BRASIL_API_URL);

    let response = fipe_service
        .get_vehicle_request(fipe_code, reference_table)
        .await?;

    let body = response.text().await.unwrap();
    let vehicle: Vec<Vehicle> = serde_json::from_str(&body).unwrap();

    Ok(vehicle)
}

/// #### `get_reference_tables()`
/// Lista as tabelas de referência existentes.
///
/// ## Retorno
/// * `Result<Vec<ReferenceTable>, Error>`
///
/// # Exemplo
/// ```rust
/// use brasilapi::fipe;
///
/// #[tokio::main]
/// async fn main() {
///    let reference_tables = fipe::get_reference_tables().await.unwrap();
/// }
/// ```
pub async fn get_reference_tables() -> Result<Vec<ReferenceTable>, Error> {
    let fipe_service = FipeService::new(BRASIL_API_URL);

    let response = fipe_service.get_reference_tables_request().await?;

    let body = response.text().await.unwrap();
    let reference_tables: Vec<ReferenceTable> = serde_json::from_str(&body).unwrap();

    Ok(reference_tables)
}

#[cfg(test)]
mod fipe_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_brands() {
        let brands = get_brands(VehicleType::Car, None).await.unwrap();
        assert!(!brands.is_empty());
    }

    #[tokio::test]
    async fn test_get_vehicles() {
        let vehicles = get_vehicles("008274-0", None).await.unwrap();
        assert!(!vehicles.is_empty());

        let vehicle = vehicles.first().unwrap();

        assert_eq!(vehicle.marca, "Audi");
        assert_eq!(vehicle.modelo, "RS E-TRON GT Quattro Aut. (Elétrico)");
    }
}
