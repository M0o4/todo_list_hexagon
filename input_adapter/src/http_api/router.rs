use crate::http_api::controller;
use axum::Router;
use axum::routing::{delete, get, post, put};
use domain::input_ports::task_command::TaskCommand;
use domain::input_ports::task_query::TaskQuery;
use std::sync::Arc;
use swagger::swagger_router;
use tower_http::trace::TraceLayer;

pub fn get_router<I>(ctx: Arc<I>) -> Router
where
    I: TaskCommand + TaskQuery,
{
    let router = Router::new()
        .route("/task/create/", post(controller::create))
        .route("/task/list/", get(controller::list))
        .route("/task/update/{task_id}", put(controller::update))
        .route("/task/{task_id}", get(controller::get_one))
        .route("/task/delete/{task_id}", delete(controller::delete))
        .with_state(ctx);

    let nest_router = swagger_router()
        .nest("/api/v1", router)
        .layer(TraceLayer::new_for_http());

    nest_router
}
