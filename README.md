<div align="center">
<h1>
<img src="https://raw.githubusercontent.com/BrasilAPI/BrasilAPI/main/public/brasilapi-logo-small.png" />

<div>

[![downloads](https://img.shields.io/crates/dv/brasilapi?label=downloads%20)](https://crates.io/crates/brasilapi)
[![version](https://img.shields.io/crates/v/brasilapi?label=version%20)](https://crates.io/crates/brasilapi)
![license](https://img.shields.io/crates/l/brasilapi)

</div>
</h1>

</div>

Uma [lib](https://crates.io/crates/brasilapi) para a API do [BrasilAPI](https://github.com/BrasilAPI/BrasilAPI) (para o Rust)

# Features
 - [x] **Bank**
 - [ ] **Cambio**
 - [x] **CEP (Zip code)**
 - [x] **CNPJ**
 - [x] **Corretoras (CVM)**
 - [ ] **CPTEC**
 - [x] **DDD**
 - [x] **Feriados Nacionais**
 - [x] **Tabela FIPE**
 - [x] **IBGE**
 - [x] **ISBN**
 - [ ] **NCM**
 - [x] **PIX**
 - [x] **Registros de domínios br**
 - [x] **Taxas**

# Como contribuir
Veja [CONTRIBUTING.md](./CONTRIBUTING.md) para ver como contribuir com o projeto.


# Instalação
Adicione a seguinte linha ao seu `Cargo.toml`:

```toml
[dependencies]
brasilapi = "0.8.0"
```

# Exemplos
Atualmente o brasilapi-rs utiliza `async/await` para fazer as requisições, então você precisa de um runtime async para rodar o código, como o [tokio](https://crates.io/crates/tokio).


```rust
use brasilapi::cep;

#[tokio::main]
async fn main() {
    let cep = cep::get_cep("01001000").await.unwrap();

    println!("Estado: {}", cep.state);
    println!("Cidade: {}", cep.city);
    println!("Bairro: {}", cep.neighborhood);
    println!("Rua: {}", cep.street);
    println!("Service: {}", cep.service);

    // Verificar se o CEP é válido
    let is_valid = cep::validate("01001000").await.unwrap();
    println!("CEP é válido: {}", is_valid);
}
```

# Documentação
Veja a documentação completa em [docs.rs](https://docs.rs/brasilapi)


# Autor
<div align="center">

| [<img src="https://github.com/peeeuzin.png?size=115" width=115><br><sub>@peeeuzin</sub>](https://github.com/peeeuzin) |
| :-------------------------------------------------------------------------------------------------------------------: |


</div>

# License
[MIT](./LICENSE)