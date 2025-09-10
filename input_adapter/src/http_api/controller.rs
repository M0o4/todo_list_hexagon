use crate::http_api::errors::ApiError;
use crate::http_api::models::create_input::CreateInput;
use crate::http_api::models::create_output::CreateOutput;
use crate::http_api::models::get_one_output::GetOneOutput;
use crate::http_api::models::list_output::ListOutput;
use crate::http_api::models::update_input::UpdateInput;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use domain::input_ports::task_command::TaskCommand;
use domain::input_ports::task_query::TaskQuery;
use domain::models::task::TaskId;
use domain::models::task_command;
use std::sync::Arc;
use ulid::Ulid;

pub async fn create<I>(
    State(use_case): State<Arc<I>>,
    Json(input): Json<CreateInput>,
) -> Result<(StatusCode, Json<CreateOutput>), ApiError>
where
    I: TaskCommand,
{
    let x = use_case.create(input.content).await?;

    Ok((StatusCode::OK, Json(CreateOutput { id: x })))
}

pub async fn update<I>(
    State(use_case): State<Arc<I>>,
    Path(task_id): Path<Ulid>,
    Json(input): Json<UpdateInput>,
) -> Result<StatusCode, ApiError>
where
    I: TaskCommand,
{
    use_case
        .update(task_command::Task::new(
            TaskId(task_id),
            input.content,
            input.completed,
        ))
        .await?;

    Ok(StatusCode::OK)
}

pub async fn delete<I>(
    State(use_case): State<Arc<I>>,
    Path(task_id): Path<Ulid>,
) -> Result<StatusCode, ApiError>
where
    I: TaskCommand,
{
    use_case.delete(TaskId(task_id)).await?;

    Ok(StatusCode::OK)
}

pub async fn list<I>(
    State(use_case): State<Arc<I>>,
) -> Result<(StatusCode, Json<Vec<ListOutput>>), ApiError>
where
    I: TaskQuery,
{
    let x = use_case.get_list().await?;

    let list = x
        .into_iter()
        .map(|x| ListOutput {
            id: TaskId(x.task_id().0),
            created_at: x.created_at().0.date().to_string(),
            completed: x.completed(),
            content: x.content().to_string(),
        })
        .collect();

    Ok((StatusCode::OK, Json(list)))
}

pub async fn get_one<I>(
    State(use_case): State<Arc<I>>,
    Path(task_id): Path<Ulid>,
) -> Result<(StatusCode, Json<GetOneOutput>), ApiError>
where
    I: TaskQuery,
{
    let x = use_case.get(TaskId(task_id)).await?;

    let output = GetOneOutput {
        id: TaskId(x.task_id().0),
        created_at: x.created_at().0.date().to_string(),
        completed: x.completed(),
        content: x.content().to_string(),
    };

    Ok((StatusCode::OK, Json(output)))
}
