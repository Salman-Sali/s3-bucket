use std::time::Duration;

use aws_sdk_s3::{Client, error::SdkError, presigning::PresigningConfig, primitives::ByteStream};
use bytes::Bytes;

use crate::{
    error::Error, s3_object::S3Object, traits::{has_content_type::HasContentType, has_key::HasKey, key_builder::KeyBuilder}
};

pub struct S3Bucket<'a> {
    pub bucket_name: String,
    pub client: &'a Client,
}

impl<'a> S3Bucket<'a> {
    pub fn new(bucket_name: String, client: &'a Client) -> Self {
        Self {
            bucket_name,
            client,
        }
    }

    pub async fn put<T: HasKey + TryInto<Bytes> + HasContentType>(
        &self,
        item: T,
    ) -> Result<(), Error> {
        let key = item.get_key();
        let bytes: Bytes = item.try_into().map_err(|_| Error::TryIntoByteError)?;
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(ByteStream::from(bytes))
            .content_type(T::get_content_type())
            .send()
            .await
            .map_err(|e| Error::PutError(e))?;

        return Ok(());
    }

    pub async fn get_with_partial_keys<
        T: KeyBuilder + TryFrom<S3Object, Error = impl std::fmt::Debug>,
    >(
        &self,
        partial_keys: Vec<Box<dyn std::fmt::Display + Send>>,
    ) -> Result<T, Error> {
        let key = T::build_key(partial_keys);
        self.get(key).await
    }

    pub async fn get<T: TryFrom<S3Object, Error = impl std::fmt::Debug>>(
        &self,
        key: String,
    ) -> Result<T, Error> {
        let result = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(&key)
            .send()
            .await
            .map_err(|e| Error::GetError(e))?;

        let bytes = result
            .body
            .collect()
            .await
            .map_err(|_| Error::ByteStreamCollectionError)?
            .into_bytes();

        let object = S3Object::new(bytes, key);

        T::try_from(object).map_err(|e| {
            eprintln!("{:?}", e);
            Error::TryFromByteError
        })
    }

    pub async fn get_maybe_with_partial_keys<
        T: KeyBuilder + TryFrom<S3Object, Error = impl std::fmt::Debug>,
    >(
        &self,
        partial_keys: Vec<Box<dyn std::fmt::Display + Send>>,
    ) -> Result<Option<T>, Error> {
        let key = T::build_key(partial_keys);
        self.get_maybe(key).await
    }

    pub async fn get_maybe<T: TryFrom<S3Object, Error = impl std::fmt::Debug>>(
        &self,
        key: String,
    ) -> Result<Option<T>, Error> {
        let result = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(&key)
            .send()
            .await;

        let result = match result {
            Ok(x) => x,
            Err(e) => {
                match e {
                    SdkError::ServiceError(e) => {
                        if e.err().is_no_such_key() {
                            return Ok(None);
                        }
                    }
                    e => {
                        return Err(Error::GetError(e));
                    }
                }
                todo!()
            }
        };

        let bytes = result
            .body
            .collect()
            .await
            .map_err(|_| Error::ByteStreamCollectionError)?
            .into_bytes();

        let s3_object = S3Object::new(bytes, key);

        T::try_from(s3_object)
            .map_err(|e| {
                eprintln!("{:?}", e);
                Error::TryFromByteError
            })
            .map(|x| Some(x))
    }

    pub async fn delete_with_partial_keys<T: KeyBuilder>(
        &self,
        partial_keys: Vec<Box<dyn std::fmt::Display + Send>>,
    ) -> Result<(), Error> {
        let key = T::build_key(partial_keys);
        self.delete(key).await
    }

    pub async fn delete(&self, key: String) -> Result<(), Error> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| Error::DeleteError(e))
    }

    pub async fn generate_presigned_url(
        &self,
        key: String,
        lifetime_duration: Duration,
    ) -> Result<String, Error> {
        let result = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .presigned(
                PresigningConfig::expires_in(lifetime_duration)
                    .map_err(|e| Error::PresigningConfigError(e))?,
            )
            .await
            .map_err(|e| Error::GetError(e))?;
        Ok(result.uri().to_string())
    }
}
