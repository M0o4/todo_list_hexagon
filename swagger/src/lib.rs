use axum::Router;
use axum::response::Redirect;
use axum::routing::{get, get_service};
use std::path::PathBuf;
use tower_http::services::ServeDir;

pub fn swagger_router() -> Router {
    let swagger_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("public")
        .join("swagger");

    println!("swagger root path: {:?}", swagger_dir);
    Router::new()
        .route("/docs", get(|| async { Redirect::temporary("/docs/") }))
        .nest_service("/docs/", get_service(ServeDir::new(swagger_dir)))
}
