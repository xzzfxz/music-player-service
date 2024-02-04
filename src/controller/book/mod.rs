use crate::service::book;
use actix_web::{get, Responder};

#[utoipa::path(
  context_path = "/api/v1/book",
  tag = "书源去重",
  responses(
      (status = 200)
  )
)]
#[get("/delete_repeat")]
pub async fn delete_repeat() -> impl Responder {
    match book::delete_repeat() {
        Ok(_) => println!("去重成功"),
        Err(info) => println!("发生错误: {:#?}", info),
    }
    "Ok".to_string()
}
