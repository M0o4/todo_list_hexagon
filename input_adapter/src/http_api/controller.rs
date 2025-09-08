use crate::http_api::errors::ApiError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use domain::input_ports::task_command::TaskCommand;
use domain::input_ports::task_query::TaskQuery;
use domain::models::task::TaskId;
use domain::models::task_query::Task;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub async fn create<I>(
    State(use_case): State<Arc<I>>,
    Json(input): Json<CreateInput>,
) -> Result<(StatusCode, Json<CreateOutput>), ApiError>
where
    I: TaskCommand,
{
    let x = use_case.create(input.text).await?;

    Ok((StatusCode::OK, Json(CreateOutput { task_id: x })))
}

pub async fn list<I>(
    State(use_case): State<Arc<I>>,
) -> Result<(StatusCode, Json<Vec<Task>>), ApiError>
where
    I: TaskQuery,
{
    let x = use_case.get_list().await?;

    Ok((StatusCode::OK, Json(x)))
}

#[derive(Debug, Deserialize)]
pub struct CreateInput {
    text: String,
}

#[derive(Debug, Serialize)]
pub struct CreateOutput {
    task_id: TaskId,
}
