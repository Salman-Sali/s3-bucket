#[derive(Debug, Clone)]
pub struct S3Object {
    pub bytes: bytes::Bytes,
    pub key: String
}

impl S3Object {
    pub fn new(bytes: bytes::Bytes, key: String) -> Self {
        Self { bytes, key }
    }
}