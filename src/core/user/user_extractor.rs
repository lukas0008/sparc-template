use axum::{extract::FromRequestParts, RequestPartsExt};
use axum_extra::extract::CookieJar;
use clerk_rs::validators::authorizer::ClerkRequest;
use hyper::{HeaderMap, StatusCode};
use log::info;
use tower_request_id::RequestId;
use uuid::Uuid;

use super::initialize_user::initialize_user;
use crate::AppState;

pub struct UserExtractor(Uuid);


// Request object to use for clerk_rs' ClerkRequest trait
struct PerchanceAuthenticatedReq<'a> {
    headers: &'a HeaderMap,
    cookies: Option<CookieJar>,
}

impl<'a> ClerkRequest for PerchanceAuthenticatedReq<'a> {
    fn get_cookie(&self, key: &str) -> Option<String> {
        match self.cookies {
            None => None,
            Some(ref jar) => jar.get(key).map(|value| value.to_string()),
        }
    }
    fn get_header(&self, key: &str) -> Option<String> {
        self.headers
            .get(key)
            .map(|v| v.to_str().ok().map(|v| v.to_string()))
            .flatten()
    }
}

impl FromRequestParts<AppState> for UserExtractor {
    type Rejection = StatusCode;
    fn from_request_parts<'life0, 'life1, 'async_trait>(
        parts: &'life0 mut axum::http::request::Parts,
        state: &'life1 AppState,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self, Self::Rejection>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let jar = parts.extract::<CookieJar>().await.ok();
            // because clerk_rs' axum implementation requires the entire request, which would consume the body here, I need to make a dummy request object which implements the trait clerk_rs provides
            let res = state
                .clerk_authorizer
                .authorize(&PerchanceAuthenticatedReq {
                    headers: &parts.headers,
                    cookies: jar,
                })
                .await;

            let res = match res {
                Ok(jwt) => jwt,
                Err(_) => return Err(StatusCode::UNAUTHORIZED),
            };

            let req_id = parts
                .extensions
                .get::<RequestId>()
                .map(ToString::to_string)
                .unwrap_or_else(|| "unknown".into());
            // initialize_user either grabs the uuid 
            let id = initialize_user(res.sub, state).await?;
            info!("(id: {}) User authenticated: {}", req_id, id);

            Ok(UserExtractor(id))
        })
    }
}
