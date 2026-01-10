#![allow(unused)]
#![allow(dead_code)]

use aws_config::{meta::region::RegionProviderChain, Region};
use s3_bucket::{traits::has_key::HasKey, S3BucketItem, S3Context};
use strum::{Display, EnumString};

pub async fn test() {    
    let config = aws_config::from_env().load().await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    let s3_context = S3Context::new(s3_client);

    let new_build = MyAppBuild {
        target_os: TargetOs::Windows,
        target_arch: TargetArch::X86_64,
        version: "0.1.0".into(),
        name: "my-app.exe".into(),
        file: File::FilePath("/home/myuser/build_folder/my-app.exe".into()),
    };
    let key = new_build.get_key();
    let _ = s3_context.put(new_build).await;
    let my_build = s3_context.get::<MyAppBuild>(key).await;
}

#[derive(S3BucketItem, Clone, Debug)]
#[s3_item_prop(bucket = get_bucket_name())]
#[s3_item_prop(key = "my-app-builds/{target_os}/{target_arch}/{version}/{name}")]
#[s3_item_prop(content_type = "application/octet-stream")]
pub struct MyAppBuild {
    pub target_os: TargetOs,
    pub target_arch: TargetArch,
    pub version: String,
    pub name: String,
    pub file: File,
}

#[derive(Display, EnumString, Clone, Debug)]
pub enum TargetOs {
    Windows,    
    Linux,
}

#[derive(Display, EnumString, Clone, Debug)]
pub enum TargetArch {
    X86_64,
    ARM64,
}

#[derive(Clone, Debug)]
pub enum File {
    FilePath(String),
    FileContent(s3_bucket::bytes::Bytes),
}

pub fn get_bucket_name() -> String {
    String::from("myBucketName")
}

impl TryInto<s3_bucket::bytes::Bytes> for MyAppBuild {
    type Error = s3_bucket::error::Error;
    fn try_into(self) -> Result<s3_bucket::bytes::Bytes, Self::Error> {
        let content = match self.file {
            File::FilePath(x) => {
                let file_content =
                    std::fs::read(x).map_err(|_| s3_bucket::error::Error::TryIntoByteError)?;
                s3_bucket::bytes::Bytes::from_owner(file_content)
            }
            File::FileContent(bytes) => bytes,
        };

        Ok(content)
    }
}



impl TryFrom<s3_bucket::s3_object::S3Object> for MyAppBuild {
    type Error = s3_bucket::error::Error;
    fn try_from(value: s3_bucket::s3_object::S3Object) -> Result<Self, Self::Error> {
        let mut parts = value.key.split('/');

        let _prefix = parts.next();
        let target_os = parts
            .next()
            .ok_or(s3_bucket::error::Error::Other("Parse error.".into()))?
            .parse::<TargetOs>()
            .map_err(|_| s3_bucket::error::Error::Other("Parse error.".into()))?;

        let target_arch = parts
            .next()
            .ok_or(s3_bucket::error::Error::Other("Parse error.".into()))?
            .parse::<TargetArch>()
            .map_err(|_| s3_bucket::error::Error::Other("Parse error.".into()))?;

        let version = parts
            .next()
            .ok_or(s3_bucket::error::Error::Other("Parse error.".into()))?
            .to_string();

        let name = parts
            .next()
            .ok_or(s3_bucket::error::Error::Other("Parse error.".into()))?
            .to_string();

        Ok(MyAppBuild {
            target_os,
            target_arch,
            version,
            name,
            file: File::FileContent(value.bytes),
        })
    }
}
