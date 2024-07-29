use crate::setup;
use clerk_rs::{clerk::Clerk, validators::authorizer::ClerkAuthorizer};
use lazy_static::lazy_static;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub _clerk_client: Clerk,
    pub clerk_authorizer: ClerkAuthorizer,
}

lazy_static! {
    pub static ref STATE: AppState = make_state();
}

fn make_state() -> AppState {
    tokio::runtime::Handle::current().block_on(async {
        let clerk_client = setup::setup_clerk().expect("Clerk could not intialize");
        let db = setup::connect_db()
            .await
            .expect("Postgres connection could not be estabilished");

        AppState {
            db,
            clerk_authorizer: clerk_rs::validators::authorizer::ClerkAuthorizer::new(
                clerk_client.clone(),
                false,
            ),
            _clerk_client: clerk_client,
        }
    })
}
