pub async fn authed_route(_user: crate::core::user::UserExtractor) -> &'static str {
  "You have been authenticated on this route!"
}