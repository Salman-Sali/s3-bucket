# s3-bucket
This is a work in progress project.


```toml
[dependencies]
s3-bucket = { git = "https://github.com/Salman-Sali/s3-bucket.git", tag = "0.2.5" }
```

Or from crates.io: `s3-bucket = "0.2.5"`.

## Cache-Control

`#[s3_item_prop(cache_control = "...")]` (needs `content_type`) sends a `Cache-Control` header
with every `put`. `put_with_cache_control(item, Some(..))` overrides it for one upload, and
`put_with_cache_control(item, None)` sends none.

```rust
#[derive(serde::Serialize, serde::Deserialize, Clone, S3BucketItem, JsonItem)]
#[s3_item_prop(bucket = get_bucket_name())]
#[s3_item_prop(key = "data/{tenant_id}/site.json")]
#[s3_item_prop(content_type = "application/json")]
#[s3_item_prop(cache_control = "max-age=60")]
pub struct SiteSettings {
    pub tenant_id: String,
    pub name: String,
}

s3_context.put(settings.clone()).await?; // Cache-Control: max-age=60
s3_context
    .put_with_cache_control(settings, Some("no-cache".into()))
    .await?; // Cache-Control: no-cache
```

## Example

Below example is for a build release. Refer `examples/file_and_json/src/json_insert_update.rs` for json example.

```rust 
use s3_bucket::{traits::has_key::HasKey, S3BucketItem, S3Context};
use strum::{Display, EnumString};

pub async fn test(s3_context: S3Context) {
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

#[derive(Clone, S3BucketItem)]
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

#[derive(Display, EnumString, Clone)]
pub enum TargetOs {
    Windows,    
    Linux,
}

#[derive(Display, EnumString, Clone)]
pub enum TargetArch {
    X86_64,
    ARM64,
}

#[derive(Clone)]
pub enum File {
    FilePath(String),
    FileContent(s3_bucket::bytes::Bytes),
}

pub fn get_bucket_name() -> String {
    String::from("MyBucket")
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

```
