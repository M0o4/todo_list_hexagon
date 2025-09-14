use axum::Router;
use axum::response::Redirect;
use axum::routing::{get, get_service};
use std::path::PathBuf;
use tower_http::services::ServeDir;

pub fn swagger_router() -> Router {
    let swagger_dir = std::env::var("SWAGGER_ASSETS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("public")
                .join("swagger")
        });

    Router::new()
        .route("/docs", get(|| async { Redirect::temporary("/docs/") }))
        .nest_service("/docs/", get_service(ServeDir::new(swagger_dir)))
}
