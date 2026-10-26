use s3_bucket::{JsonItem, S3BucketItem, S3Context};

#[allow(dead_code)]
async fn json_insert_update() {
    #[allow(deprecated)]
    let config = aws_config::from_env().load().await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    let s3_context = S3Context::new(s3_client);

    let post_id = String::from("123");

    let _ = s3_context.put(PostComments::new(post_id.clone())).await;
    let _ = s3_context
        .get_with_partial_keys::<PostComments>(vec![Box::new(post_id)])
        .await;

    // Sent with `Cache-Control: max-age=60` (from the attribute)...
    let settings = SiteSettings {
        tenant_id: String::from("tenant-1"),
        name: String::from("My store"),
    };
    let _ = s3_context.put(settings.clone()).await;
    // ...or with a different value for this one upload.
    let _ = s3_context
        .put_with_cache_control(settings, Some(String::from("no-cache")))
        .await;
}

pub fn get_bucket_name() -> String {
    String::from("MyBucket")
}

#[derive(serde::Serialize, serde::Deserialize, Clone, S3BucketItem, JsonItem)]
#[s3_item_prop(bucket = get_bucket_name())]
#[s3_item_prop(key = "posts/{post_id}/comments.json")]
#[s3_item_prop(content_type = "application/json")]
pub struct PostComments {
    pub post_id: String,
    pub comments: Vec<String>,
}

impl PostComments {
    pub fn new(post_id: String) -> Self {
        Self {
            post_id,
            comments: vec![],
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, S3BucketItem, JsonItem)]
#[s3_item_prop(bucket = get_bucket_name())]
#[s3_item_prop(key = "data/{tenant_id}/site.json")]
#[s3_item_prop(content_type = "application/json")]
#[s3_item_prop(cache_control = "max-age=60")]
pub struct SiteSettings {
    pub tenant_id: String,
    pub name: String,
}
