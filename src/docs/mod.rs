use crate::controller;
use crate::model;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(controller::user::login, controller::book::delete_repeat),
    components(schemas(model::user::LoginBody, model::book::SourceItem))
)]
pub struct ApiDoc;
