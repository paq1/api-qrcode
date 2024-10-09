use crate::api::qrcode::services::{ImageServiceImpl, QrcodeGeneratorImpl};
use crate::core::qrcode::services::image_service::ImageService;
use crate::core::qrcode::services::qrcode_generator::QrcodeGenerator;
use image::{ImageBuffer, Luma};
use qrcode::QrCode;
use std::sync::Arc;

pub struct QrcodeComponent {
    pub qrcode_generator: Arc<dyn QrcodeGenerator<QrCode>>,
    pub image_service: Arc<dyn ImageService<QrCode, ImageBuffer<Luma<u8>, Vec<u8>>>>,
}

impl QrcodeComponent {
    pub async fn new() -> Self {
        let qrcode_generator = Arc::new(
            QrcodeGeneratorImpl {}
        );

        let image_service = Arc::new(
            ImageServiceImpl {}
        );


        Self {
            qrcode_generator,
            image_service,
        }
    }
}