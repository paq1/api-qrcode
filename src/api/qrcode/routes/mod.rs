use crate::core::qrcode::services::image_service::ImageService;
use crate::core::qrcode::services::qrcode_generator::QrcodeGenerator;
use crate::models::qrcode::commands::CreateQrcodeCommand;
use actix_web::{post, web, HttpResponse, Responder};
use image::{ImageBuffer, ImageFormat, Luma};
use qrcode::QrCode;
use std::io::Cursor;
use std::sync::Arc;

#[utoipa::path(
    tag = "qrcode",
    path = "/qrcode/generate-image",
    request_body = CreateQrcodeCommand,
    responses(
        (
        status = 200,
        description = "Get the current state.",
        body = String
        )
    )
)]
#[post("/generate-image")]
pub async fn generate_qrcode_image(
    body: web::Json<CreateQrcodeCommand>,
    qrcode_generator: web::Data<Arc<dyn QrcodeGenerator<QrCode>>>,
    qrcode_image_service: web::Data<Arc<dyn ImageService<QrCode, ImageBuffer<Luma<u8>, Vec<u8>>>>>,
) -> impl Responder {

    let qrcode = qrcode_generator.create_qrcode(&body.data);
    let image: ImageBuffer<Luma<u8>, Vec<u8>> = qrcode_image_service.create_qrcode_image(&qrcode);

    // Créer un buffer en mémoire où l'on va écrire l'image PNG
    let mut buffer = Cursor::new(Vec::new());

    // Encoder l'image en PNG et écrire dans le buffer
    if image.write_to(&mut buffer, ImageFormat::Png).is_err() {
        return HttpResponse::InternalServerError().body("Could not encode image");
    }

    // Obtenir le contenu encodé du buffer
    let image_data = buffer.into_inner();

    HttpResponse::Ok()
        .insert_header(("Content-Type", mime::IMAGE_PNG.to_string()))
        // .insert_header(("Content-Disposition", "attachment; filename=\"image.png\""))
        .body(image_data)

}
