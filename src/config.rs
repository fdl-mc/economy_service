#[derive(serde::Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub users_service_url: String,
}
