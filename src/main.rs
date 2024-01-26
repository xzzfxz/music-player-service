use std::error::Error;

use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod controller;
mod db;
mod docs;
mod middleware;
mod model;
mod schema;
mod service;

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    HttpServer::new(move || {
        App::new()
            .configure(controller::user_config)
            .configure(controller::book_config)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", docs::ApiDoc::openapi()),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
