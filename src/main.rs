use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


use crate::api::swagger::ApiDoc;


use api_qrcode::api::qrcode::qrcode_component::QrcodeComponent;
use api_qrcode::api::qrcode::routes::generate_qrcode_image;
use framework_cqrs_lib::cqrs::models::errors::StandardHttpError;
use log::info;

mod core;
mod api;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("lancement du server");
    // qrcode aggregat
    let qrcode_component = QrcodeComponent::new().await;

    let openapi = ApiDoc::openapi();
    let api_address = std::env::var("API_ADDRESS").unwrap();
    let api_port = std::env::var("API_PORT").unwrap().parse::<u16>().unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .supports_credentials();

        let standard_http_error = StandardHttpError::new();

        App::new()
            .wrap(cors)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url(
                "/api-docs/openapi.json",
                openapi.clone(),
            ))
            .app_data(web::Data::new(standard_http_error))
            // qrcode services
            .service(
                web::scope("/qrcode")
                    .service(generate_qrcode_image)
                    .app_data(web::Data::new(Arc::clone(&qrcode_component.qrcode_generator)))
                    .app_data(
                        web::Data::new(Arc::clone(&qrcode_component.image_service))
                    )
            )
    })
        .workers(2)
        .bind((api_address.clone(), api_port.clone()))?
        .run()
        .await
}