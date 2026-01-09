use aws_sdk_s3::{config::http::HttpResponse, error::SdkError, operation::{delete_object::DeleteObjectError, get_object::GetObjectError, put_object::PutObjectError}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error during put operation.")]
    PutError(SdkError<PutObjectError, HttpResponse>),
    #[error("Error during get operation.")]
    GetError(SdkError<GetObjectError, HttpResponse>),
    #[error("Error during delete operation.")]
    DeleteError(SdkError<DeleteObjectError, HttpResponse>),
    #[error("Error while trying to convert into byte stream.")]
    TryIntoByteError,
    #[error("Error while trying to convert from byte stream.")]
    TryFromByteError,
    #[error("Empty byte stream encountered while operforming get operation.")]
    EmptyByteStream,
    #[error("Error while converting byte array to string.")]
    ByteArrayToString,
    #[error("Error while collecting bytes from ByteStream.")]
    ByteStreamCollectionError
}