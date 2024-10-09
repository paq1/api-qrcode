pub trait ImageService<QrCode, Image>: Send + Sync {
    fn create_qrcode_image(&self, qrcode: &QrCode) -> Image;
}
