use crate::model::user::LoginBody;
use crate::service::user;
use actix_web::{post, web::Json, Responder};

#[utoipa::path(
  context_path = "/api/v1/user",
  tag = "用户相关操作",
  request_body = LoginBody,
  responses(
      (status = 200, description = "登录 ", body = LoginBody)
  )
)]
#[post("/login")]
pub async fn login(params: Json<LoginBody>) -> impl Responder {
    println!("登录参数: {:#?}", params);
    match user::login(&params.username, &params.password) {
        Ok(_) => println!("登录成功"),
        Err(info) => println!("登录失败: {:#?}", info),
    };
    params
}
