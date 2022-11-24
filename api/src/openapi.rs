use economy_service_entity::economy_state::Model as EconomyState;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use routes::DataPay;

use crate::responses::AppError;
use crate::routes;

const DOCS_TEMPLATE: &'static str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Documentation</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui.css" />
</head>
<body>
<div id="ui"></div>
<script src="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui-bundle.js" crossorigin></script>
<script>
  window.onload = () => {
    window.ui = SwaggerUIBundle({
      spec: %SPEC%,
      dom_id: '#ui',
    });
  };
</script>
</body>
</html>"#;

#[derive(OpenApi, Debug)]
#[openapi(
    paths(routes::get_by_id, routes::get_self, routes::pay),
    components(schemas(EconomyState, AppError, DataPay)),
    modifiers(&SecurityAddon, &InfoAddon),
)]
pub(crate) struct ApiDoc;

impl ApiDoc {
    pub(crate) fn router() -> axum::Router {
        async fn schema_handler() -> impl axum::response::IntoResponse {
            axum::Json(ApiDoc::openapi())
        }

        async fn docs_handler() -> impl axum::response::IntoResponse {
            axum::response::Html(
                DOCS_TEMPLATE.replace("%SPEC%", &ApiDoc::openapi().to_json().unwrap()),
            )
        }

        axum::Router::new()
            .route("/openapi.json", axum::routing::get(schema_handler))
            .route("/docs", axum::routing::get(docs_handler))
    }
}

struct InfoAddon;
impl Modify for InfoAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.info = utoipa::openapi::InfoBuilder::new()
            .title("FDL Economy Service API")
            .description(Some("Core service for virtual currency management."))
            .version("0.1.0")
            .build();
    }
}

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
