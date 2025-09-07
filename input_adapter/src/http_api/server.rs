use crate::http_api::router::get_router;
use domain::input_ports::task_command::TaskCommand;
use std::sync::Arc;

pub async fn run<I: TaskCommand>(use_case: Arc<I>) -> Result<(), anyhow::Error> {
    let router = get_router(use_case);
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Listening on: {}", addr);

    axum::serve(listener, router).await?;

    Ok(())
}
