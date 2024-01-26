use actix_web::web::{scope, ServiceConfig};

pub mod book;
pub mod user;

/// 登录配置
pub fn user_config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1/user").service(user::login));
}

// 书源操作
pub fn book_config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1/book").service(book::delete_repeat));
}
