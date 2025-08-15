//! # brasilapi-rs
//!
//! Biblioteca para consulta da [BrasilAPI](https://brasilapi.com.br/) para o Rust.
//!
//! ## Exemplos
//! Atualmente o brasilapi-rs utiliza `async/await` para fazer as requisições, então você precisa de um runtime async para rodar o código, como o [tokio](https://crates.io/crates/tokio).
//! ```rust
//! use brasilapi::cep;
//!
//! #[tokio::main]
//! async fn main() {
//!     let cep = cep::get_cep("01001000").await.unwrap();
//!
//!     println!("Estado: {}", cep.state);
//!     println!("Cidade: {}", cep.city);
//!     println!("Bairro: {}", cep.neighborhood);
//!     println!("Rua: {}", cep.street);
//!     println!("Service: {}", cep.service);
//!
//!     // Verificar se o CEP é válido
//!     let is_valid = cep::validate("01001000").await.unwrap();
//!     println!("CEP é válido: {}", is_valid);
//! }
//! ```
//!
//! ## Módulos
//! A biblioteca é dividida em módulos, cada um com sua responsabilidade:
//! * [Bank](bank/index.html) - Informações sobre sistema bancário brasileiro.
//! * [Cep](cep/index.html) - Informações referentes a CEPs
//! * [Cnpj](cnpj/index.html) - Busca dados de empresas por CNPJ
//! * [Corretoras](corretoras/index.html) - Informações referentes a Corretoras ativas listadas na CVM
//! * [Ddd](ddd/index.html) - Informações relacionadas a DDDs
//! * [Fipe](fipe/index.html) - Informações sobre Preço Médio de Veículos fornecido pela FIPE (Fundação Instituto de Pesquisas Econômicas)
//! * [Holidays](holidays/index.html) - Informações sobre feriados nacionais
//! * [Ibge](ibge/index.html) - Informações sobre estados Provenientes do IBGE
//! * [Isbn](isbn/index.html) - Informações referentes a ISBNs
//! * [Pix](pix/index.html) - Informações referentes ao PIX
//! * [Registrobr](registrobr/index.html) - Avalia um dominio no registro.br
//! * [Error](error/index.html) - Estrutura de erros da biblioteca
pub mod bank;
pub mod cep;
pub mod cnpj;
pub mod corretoras;
pub mod ddd;
pub mod error;
pub mod fipe;
pub mod holidays;
pub mod ibge;
pub mod isbn;
pub mod ncm;
pub mod pix;
pub mod registrobr;
pub mod taxas;

pub mod spec;
