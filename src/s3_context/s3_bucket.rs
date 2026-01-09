use aws_sdk_s3::{error::SdkError, primitives::ByteStream, Client};
use bytes::Bytes;

use crate::{
    error::Error,
    traits::{has_content_type::HasContentType, has_key::HasKey, key_builder::KeyBuilder},
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

    pub async fn get_with_partial_key<
        T: KeyBuilder + TryFrom<Bytes, Error = impl std::fmt::Debug>,
    >(
        &self,
        partial_key: String,
    ) -> Result<T, Error> {
        let key = T::build_key(&partial_key);
        self.get(key).await
    }

    pub async fn get<T: TryFrom<Bytes, Error = impl std::fmt::Debug>>(
        &self,
        key: String,
    ) -> Result<T, Error> {
        let result = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .map_err(|e| Error::GetError(e))?;

        let bytes = result
            .body
            .collect()
            .await
            .map_err(|_| Error::ByteStreamCollectionError)?
            .into_bytes();

        T::try_from(bytes).map_err(|e| {
            eprintln!("{:?}", e);
            Error::TryFromByteError
        })
    }

    pub async fn get_maybe_with_partial_key<
        T: KeyBuilder + TryFrom<Bytes, Error = impl std::fmt::Debug>,
    >(
        &self,
        partial_key: String,
    ) -> Result<Option<T>, Error> {
        let key = T::build_key(&partial_key);
        self.get_maybe(key).await
    }

    pub async fn get_maybe<T: TryFrom<Bytes, Error = impl std::fmt::Debug>>(
        &self,
        key: String,
    ) -> Result<Option<T>, Error> {
        let result = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await;

        let result = match result {
            Ok(x) => x,
            Err(e) => {
                match e {
                    SdkError::ServiceError(e) => if e.err().is_no_such_key() {
                        return Ok(None)
                    }
                    e => {
                        return Err(Error::GetError(e));
                    },
                }
                todo!()
            },
        };

        let bytes = result
            .body
            .collect()
            .await
            .map_err(|_| Error::ByteStreamCollectionError)?
            .into_bytes();

        T::try_from(bytes).map_err(|e| {
            eprintln!("{:?}", e);
            Error::TryFromByteError
        }).map(|x| Some(x))
    }

    pub async fn delete_with_partial_key<T: KeyBuilder>(
        &self,
        partial_key: String,
    ) -> Result<(), Error> {
        let key = T::build_key(&partial_key);
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
}
