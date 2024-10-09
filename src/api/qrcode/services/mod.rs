use image::{ImageBuffer, Luma};
use qrcode::QrCode;
use crate::core::qrcode::services::image_service::ImageService;
use crate::core::qrcode::services::qrcode_generator::QrcodeGenerator;

pub struct QrcodeGeneratorImpl;

impl QrcodeGenerator<QrCode> for QrcodeGeneratorImpl {
    fn create_qrcode(&self, message: &str) -> QrCode {
        QrCode::new(message).unwrap()
    }
}

pub struct ImageServiceImpl;

impl ImageService<QrCode, ImageBuffer<Luma<u8>, Vec<u8>>> for ImageServiceImpl {
    fn create_qrcode_image(&self, qrcode: &QrCode) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        qrcode.render::<Luma<u8>>().build()
    }
}
