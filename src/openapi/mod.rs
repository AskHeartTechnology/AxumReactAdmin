mod user;

use utoipa::{
    Modify, OpenApi,
    openapi::{
        Components,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if openapi.components.is_none() {
            openapi.components = Some(Components::new());
        }

        openapi
            .components
            .as_mut()
            .expect("components 已初始化")
            .add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
    }
}

use crate::openapi::user::UserApiDoc;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Axum React Admin Api Docs",
        description = "ARA 接口文档",
        version = "1.0.0"
    ),
    modifiers(&SecurityAddon),
    nest(
        (path = "/api", api = UserApiDoc),
    )
)]
pub struct ApiDoc;
