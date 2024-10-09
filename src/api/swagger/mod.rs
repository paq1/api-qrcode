use crate::api::qrcode::routes::__path_generate_qrcode_image;
use crate::models::qrcode::commands::CreateQrcodeCommand;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        generate_qrcode_image
    ),
    components(
        schemas(
            CreateQrcodeCommand
        )
    ),
    modifiers(& SecurityAddon)
)]
pub struct ApiDoc;

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build()
            ),
        );
        components.add_security_scheme(
            "api_key_auth",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-API-KEY"))),
        )
    }
}