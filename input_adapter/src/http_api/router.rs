use crate::http_api::controller;
use axum::Router;
use axum::routing::{get, post};
use domain::input_ports::task_command::TaskCommand;
use domain::input_ports::task_query::TaskQuery;
use std::sync::Arc;

pub fn get_router<I>(ctx: Arc<I>) -> Router
where
    I: TaskCommand + TaskQuery,
{
    let router = Router::new()
        .route("/api/v1/task/create/", post(controller::create))
        .route("/api/v1/task/list/", get(controller::list))
        .with_state(ctx);

    router
}
