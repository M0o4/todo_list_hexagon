use crate::http_api::controller;
use axum::Router;
use axum::routing::{get, post};
use domain::input_ports::task_command::TaskCommand;
use std::sync::Arc;

pub fn get_router<I: TaskCommand>(ctx: Arc<I>) -> Router {
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/v1/task/create/", post(controller::create))
        .with_state(ctx);

    router
}
