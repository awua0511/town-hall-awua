pub mod auth;
mod helper;
pub mod guild;
pub mod quest;
pub mod solution;
pub mod user;

use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
};

pub async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        "Welcome to our Town Hall health center!\n\
        We heal your vibe coded ego back to perfect health!\n\
        Shall we process your projects?\n\
        ...\n\
        Ok. We'll need your repo.\n\
        ...\n\
        Di-di-di-di-ding! ♪\n\
        ...\n\
        Thank you! Your codebases are fighting fit!\n\
        We hope to see you again!",
    )
}

pub async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, uri.to_string())
}
