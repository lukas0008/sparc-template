# My personal Sqlx Postgres Axum Rust Clerk Template

## Logging

`/src/setup.rs::setup_logger`
This project sets up logging with the tracing and log crate, by redirecting all tracing logs to log.
Logs are also written to a debug.log file, when in debug mode.

## Auth

Auth is handled by the `UserExtractor` in `/src/core/user/user_extractor.rs`. Whenever it tries to authenticate, it calls the `ClerkAuthenticator::authorize` method to get the clerk id.

Note: Users are not added to the db when they register (using something like clerk webhooks), instead they are added lazily on the first request that includes the `UserExtractor`.

## Migrations

This template uses sqlx for migrations, there is already one migration added for the user table.