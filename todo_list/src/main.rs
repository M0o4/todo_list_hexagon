use domain::use_case::TaskUseCase;
use input_adapter::http_api;
use output_adapter::sqlx::sqlx_task_command::SqlxTaskCommand;
use output_adapter::sqlx::sqlx_task_query::SqlxTaskQuery;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::fmt()
        .json()
        .with_current_span(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;

    let task_command = SqlxTaskCommand::new(pool.clone());
    let task_query = SqlxTaskQuery::new(pool.clone());
    let use_case = TaskUseCase::new(Arc::new(task_command), Arc::new(task_query));

    http_api::server::run(Arc::new(use_case)).await?;

    Ok(())
}
