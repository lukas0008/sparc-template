pub fn setup_logger() -> Result<(), fern::InitError> {
    use std::time::SystemTime;
    let logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {}: {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout());
    #[cfg(debug_assertions)]
    let logger = logger.chain(fern::log_file("debug.log")?);
    logger.apply()?;
    Ok(())
}

pub fn setup_clerk() -> Result<clerk_rs::clerk::Clerk, Box<dyn std::error::Error>> {
    use clerk_rs::{clerk::Clerk, ClerkConfiguration};

    let config = ClerkConfiguration::new(None, None, Some(std::env::var("CLERK_SECRET")?), None);

    Ok(Clerk::new(config))
}

pub async fn connect_db() -> Result<sqlx::PgPool, Box<dyn std::error::Error>> {
    Ok(sqlx::PgPool::connect(std::env::var("DATABASE_URL")?.as_str()).await?)
}

macro_rules! trace_layer {
    () => {{
        use axum::extract::Request;
        use log::info;
        use tower_http::trace::TraceLayer;
        use tower_request_id::RequestId;
        use tracing::Span;

        TraceLayer::new_for_http().on_request(|req: &Request<_>, _: &Span| {
            let id = req
                .extensions()
                .get::<RequestId>()
                .map(ToString::to_string)
                .unwrap_or_else(|| "unknown".into());
            info!("(id: {}) {} {}", id, req.method(), req.uri().path());
        })
    }};
}
pub(crate) use trace_layer;
