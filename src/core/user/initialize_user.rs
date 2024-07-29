use hyper::StatusCode;
use uuid::Uuid;

use crate::AppState;

pub async fn initialize_user(c_user_id: String, state: &AppState) -> Result<Uuid, StatusCode> {
    let u = sqlx::query!(
        "WITH ins AS (
  INSERT INTO \"user\" (id, clerkId) 
  VALUES ($1, $2)
  ON CONFLICT (clerkId) DO NOTHING
  RETURNING id
)
SELECT id FROM ins
UNION ALL
SELECT id FROM \"user\" WHERE clerkId = $2
LIMIT 1;
",
        Uuid::new_v4(),
        c_user_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let id = u.id.expect("how can there be none if i just inserted it?");

    Ok(id)
}
