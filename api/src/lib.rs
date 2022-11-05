use economy_service_core::get_or_create_economy_state;
use economy_service_entity::economy_state;
use economy_service_migration::{
    sea_orm::{Database, DbConn},
    Migrator, MigratorTrait,
};
use poem::{
    error::InternalServerError, listener::TcpListener, web::Data, EndpointExt, Request, Result,
    Route, Server,
};
use poem_openapi::{
    auth::ApiKey, payload::Json, ApiResponse, OpenApi, OpenApiService, SecurityScheme,
};
use serde::Deserialize;
use std::sync::Arc;
use users_service_client::{GetSelfResponse, User, UsersServiceClient};

#[derive(Debug, SecurityScheme)]
#[oai(
    type = "api_key",
    rename = "Authentication token",
    key_name = "x-token",
    in = "header",
    checker = "api_checker"
)]
struct UserAuth(User);

async fn api_checker(req: &Request, token: ApiKey) -> Option<User> {
    let ctx = req.data::<Arc<AppState>>().unwrap().clone();
    let client = &ctx.users_client;
    match client.get_self(token.key).await {
        Ok(res) => match res {
            GetSelfResponse::Ok(user) => Some(user),
            _ => None,
        },
        Err(_) => None,
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    database_url: String,
    users_service_url: String,
}

#[derive(Debug)]
struct AppState {
    users_client: UsersServiceClient,
    conn: DbConn,
}

#[derive(ApiResponse)]
enum GetSelfEconomyStateResponse {
    /// Returns when the query is successful
    #[oai(status = 200)]
    Ok(Json<economy_state::Model>),
}

#[derive(Debug)]
struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/me", method = "get")]
    async fn get_self(
        &self,
        user: UserAuth,
        ctx: Data<&Arc<AppState>>,
    ) -> Result<GetSelfEconomyStateResponse> {
        let state = get_or_create_economy_state(user.0.id, &ctx.conn)
            .await
            .map_err(InternalServerError)?;
        Ok(GetSelfEconomyStateResponse::Ok(Json(state)))
    }
}

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=info,sqlx=warn");
    }
    tracing_subscriber::fmt::init();

    let config = envy::from_env::<Config>().unwrap();

    let conn = Database::connect(&config.database_url).await.unwrap();
    let users_client = UsersServiceClient::new(&config.users_service_url);

    let ctx = AppState { users_client, conn };

    Migrator::up(&ctx.conn, None).await.unwrap();

    let api = OpenApiService::new(Api, "Users Service", "1.0");
    let docs = api.swagger_ui();
    let openapi_json = api.spec_endpoint();
    let openapi_yaml = api.spec_endpoint_yaml();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(
            Route::new()
                .nest("/", api)
                .nest("/docs", docs)
                .nest("/docs/openapi.json", openapi_json)
                .nest("/docs/openapi.yaml", openapi_yaml)
                .data(Arc::new(ctx)),
        )
        .await
}
