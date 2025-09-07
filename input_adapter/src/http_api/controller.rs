use crate::http_api::errors::ApiError;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use domain::input_ports::task_command::TaskCommand;
use domain::models::task::TaskId;
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

#[derive(Debug, Deserialize)]
pub struct CreateInput {
    text: String,
}

#[derive(Debug, Serialize)]
pub struct CreateOutput {
    task_id: TaskId,
}
