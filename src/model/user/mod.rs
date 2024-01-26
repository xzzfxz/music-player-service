use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!({"username": "admin", "password": "123456"}))]
pub struct LoginBody {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}
