use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod book;
pub mod user;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({"code": 200, "msg": "success", "data": null}))]
pub struct ResponseResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}
