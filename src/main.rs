use log::info;

mod core;
mod router;
mod setup;
mod routes;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
    _clerk_client: clerk_rs::clerk::Clerk,
    clerk_authorizer: clerk_rs::validators::authorizer::ClerkAuthorizer,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    setup::setup_logger()?;

    let clerk_client = setup::setup_clerk()?;
    let db = setup::connect_db().await?;

    let state = AppState {
        db,
        clerk_authorizer: clerk_rs::validators::authorizer::ClerkAuthorizer::new(
            clerk_client.clone(),
            false,
        ),
        _clerk_client: clerk_client,
    };

    let app = router::make_router(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await?;
    info!("Starting");
    axum::serve(listener, app).await?;

    Ok(())
}
