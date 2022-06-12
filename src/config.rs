#[derive(serde::Deserialize)]
pub struct Config {
    pub database_url: String,
    pub users_service_url: String,
}
