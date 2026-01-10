use std::time::Duration;

use aws_sdk_s3::Client;
use bytes::Bytes;
use s3_bucket::S3Bucket;

use crate::{
    error::Error, s3_object::S3Object, traits::{
        has_bucket_name::HasBucketName, has_content_type::HasContentType, has_key::HasKey,
        key_builder::KeyBuilder,
    }
};

pub mod s3_bucket;

#[derive(Debug)]
pub struct S3Context {
    pub client: Client,
}

impl S3Context {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn with_bucket(&'_ self, bucket_name: String) -> S3Bucket<'_> {
        S3Bucket::new(bucket_name, &self.client)
    }

    pub async fn put<T: HasKey + TryInto<Bytes> + HasContentType + HasBucketName>(
        &self,
        item: T,
    ) -> Result<(), Error> {
        self.with_bucket(T::get_bucket_name()).put(item).await
    }

    pub async fn get_with_partial_keys<
        T: KeyBuilder + TryFrom<S3Object, Error = impl std::fmt::Debug> + HasBucketName,
    >(
        &self,
        partial_keys: Vec<Box<dyn std::fmt::Display + Send>>,
    ) -> Result<T, Error> {
        self.with_bucket(T::get_bucket_name())
            .get_with_partial_keys(partial_keys)
            .await
    }

    pub async fn get<T: TryFrom<S3Object, Error = impl std::fmt::Debug> + HasBucketName>(
        &self,
        key: String,
    ) -> Result<T, Error> {
        self.with_bucket(T::get_bucket_name()).get(key).await
    }

    pub async fn get_maybe_with_partial_keys<
        T: KeyBuilder + TryFrom<S3Object, Error = impl std::fmt::Debug> + HasBucketName,
    >(
        &self,
        partial_keys: Vec<Box<dyn std::fmt::Display + Send>>,
    ) -> Result<Option<T>, Error> {
        self.with_bucket(T::get_bucket_name())
            .get_maybe_with_partial_keys(partial_keys)
            .await
    }

    pub async fn get_maybe<T: TryFrom<S3Object, Error = impl std::fmt::Debug> + HasBucketName>(
        &self,
        key: String,
    ) -> Result<Option<T>, Error> {
        self.with_bucket(T::get_bucket_name()).get_maybe(key).await
    }

    pub async fn delete_with_partial_keys<T: KeyBuilder + HasBucketName>(
        &self,
        partial_keys: Vec<Box<dyn std::fmt::Display + Send>>,
    ) -> Result<(), Error> {
        self.with_bucket(T::get_bucket_name())
            .delete_with_partial_keys::<T>(partial_keys)
            .await
    }

    pub async fn delete<T: HasBucketName>(&self, key: String) -> Result<(), Error> {
        self.with_bucket(T::get_bucket_name()).delete(key).await
    }

    pub async fn generate_presigned_url<T: HasBucketName>(
        &self,
        key: String,
        lifetime_duration: Duration,
    ) -> Result<String, Error> {
        self.with_bucket(T::get_bucket_name())
            .generate_presigned_url(key, lifetime_duration)
            .await
    }
}
