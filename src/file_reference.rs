#[derive(PartialEq, Eq, Clone, Debug)]
pub enum FileReference {
    None,
    FilePath(String),
    Content(crate::bytes::Bytes),
}

impl Default for FileReference {
    fn default() -> Self {
        FileReference::None
    }
}
