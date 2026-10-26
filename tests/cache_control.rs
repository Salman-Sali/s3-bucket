//! `cache_control` on `#[s3_item_prop(...)]`, checked against a local fake S3 endpoint that
//! records the headers of the `PutObject` request.

use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};
use s3_bucket::{JsonItem, S3BucketItem, S3Context, traits::has_content_type::HasContentType};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::mpsc,
};

fn bucket() -> String {
    String::from("test-bucket")
}

#[derive(serde::Serialize, serde::Deserialize, Clone, S3BucketItem, JsonItem)]
#[s3_item_prop(bucket = bucket())]
#[s3_item_prop(key = "data/{tenant}/site.json")]
#[s3_item_prop(content_type = "application/json")]
#[s3_item_prop(cache_control = "max-age=60")]
struct Site {
    tenant: String,
    name: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, S3BucketItem, JsonItem)]
#[s3_item_prop(bucket = bucket())]
#[s3_item_prop(key = "data/{tenant}/plain.json")]
#[s3_item_prop(content_type = "application/json")]
struct Plain {
    tenant: String,
}

/// Fake S3: answers every request with 200 and sends its lowercased request head.
async fn fake_s3() -> (String, mpsc::UnboundedReceiver<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let tx = tx.clone();
            tokio::spawn(async move {
                let mut head = Vec::new();
                let mut buf = [0u8; 4096];
                loop {
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    if n == 0 {
                        return;
                    }
                    head.extend_from_slice(&buf[..n]);
                    if let Some(end) = head.windows(4).position(|w| w == b"\r\n\r\n") {
                        let text = String::from_utf8_lossy(&head[..end]).to_lowercase();
                        let _ = tx.send(text);
                        head.clear();
                        let _ = socket
                            .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 0\r\n\r\n")
                            .await;
                    }
                }
            });
        }
    });
    (url, rx)
}

fn context(url: &str) -> S3Context {
    let config = aws_sdk_s3::Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .credentials_provider(Credentials::new("test", "test", None, None, "test"))
        .endpoint_url(url)
        .force_path_style(true)
        .build();
    S3Context::new(aws_sdk_s3::Client::from_conf(config))
}

fn cache_control_of(head: &str) -> Option<String> {
    head.lines()
        .find_map(|line| line.strip_prefix("cache-control:"))
        .map(|v| v.trim().to_string())
}

#[test]
fn derive_sets_cache_control() {
    assert_eq!(Site::get_cache_control().as_deref(), Some("max-age=60"));
    assert_eq!(Site::get_content_type(), "application/json");
    assert_eq!(Plain::get_cache_control(), None);
}

#[tokio::test]
async fn put_sends_cache_control_and_can_override_it() {
    let (url, mut requests) = fake_s3().await;
    let s3 = context(&url);
    let site = Site {
        tenant: "t1".into(),
        name: "My store".into(),
    };

    s3.put(site.clone()).await.unwrap();
    let head = requests.recv().await.unwrap();
    assert!(
        head.starts_with("put /test-bucket/data/t1/site.json"),
        "{head}"
    );
    assert_eq!(cache_control_of(&head).as_deref(), Some("max-age=60"));

    s3.put_with_cache_control(site.clone(), Some("no-cache".into()))
        .await
        .unwrap();
    let head = requests.recv().await.unwrap();
    assert_eq!(cache_control_of(&head).as_deref(), Some("no-cache"));

    s3.with_bucket(bucket())
        .put_with_cache_control(site, None)
        .await
        .unwrap();
    let head = requests.recv().await.unwrap();
    assert_eq!(cache_control_of(&head), None);

    s3.put(Plain {
        tenant: "t1".into(),
    })
    .await
    .unwrap();
    let head = requests.recv().await.unwrap();
    assert_eq!(cache_control_of(&head), None);
}
