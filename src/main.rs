use log::info;

mod app_state;
mod core;
mod router;
mod routes;
mod setup;
pub use app_state::{AppState, STATE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    setup::setup_logger()?;

    let app = router::make_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await?;
    info!("Starting");
    axum::serve(listener, app).await?;

    Ok(())
}
