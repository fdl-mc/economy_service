use economy_service_entity::economy_state::Model as EconomyState;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::responses::AppError;
use crate::routes;

#[derive(OpenApi)]
#[openapi(
    paths(routes::get_self),
    components(schemas(EconomyState, AppError)),
    modifiers(&SecurityAddon),
)]
pub(crate) struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-token"))),
            )
        }
    }
}
