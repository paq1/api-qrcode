pub trait QrcodeGenerator<QrCode>: Send + Sync {
    fn create_qrcode(&self, message: &str) -> QrCode;
}
