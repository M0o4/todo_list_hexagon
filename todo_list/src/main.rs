use std::sync::Arc;
use domain::use_case::TaskUseCase;
use input_adapter::http_api::router::get_router;
use output_adapter::task_command_mock::TaskCommandMock;

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

    let task_command = TaskCommandMock {};
    let use_case = TaskUseCase::new(Arc::new(task_command));

    let router = get_router(Arc::new(use_case));
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Listening on: {}", addr);

    axum::serve(listener, router).await?;

    Ok(())
}
