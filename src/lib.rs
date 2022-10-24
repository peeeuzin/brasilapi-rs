pub mod cep;
pub mod ddd;
pub mod spec;

#[cfg(test)]
mod tests {
    use super::cep;

    #[tokio::test]
    async fn get_cep() {
        let cep = cep::get_cep("01001000").await.unwrap();

        assert_eq!(cep.state, "SP");
        assert_eq!(cep.street, "Praça da Sé");
    }

    #[tokio::test]
    async fn get_cep_error() {
        let cep = cep::get_cep("12345678").await;

        assert!(cep.is_err());
    }

    #[tokio::test]
    async fn validate() {
        let cep = cep::validate("01001000").await;

        assert!(cep);
    }
}
