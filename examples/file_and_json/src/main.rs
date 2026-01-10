use crate::json_insert_update::PostComments;

mod file_insert_update;
mod json_insert_update;

#[tokio::main]
pub async fn main() {
    file_insert_update::test().await;
}
