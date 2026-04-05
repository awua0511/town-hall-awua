use anyhow::Result;
use axum::Router;
use axum::routing::get;
use axum::routing::post;
use axum::routing::patch;

use crate::AppState;
use crate::handlers::*;

pub async fn root() -> Result<Router<AppState>> {
    let router = Router::new()
        .fallback(fallback)
        .route("/", get(hello_world))
        .route("/health", get(health))
        .nest("/users", users().await?)
        .nest("/quests", quests().await?)
        .nest("/guilds", guilds().await?)
        .nest("/auth", auth().await?);
    Ok(router)
}

async fn users() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/{id}", get(user::get_from_url))
        .route("/me", get(user::get_me))
        .route("/resolve/{handle}", get(user::resolve_handle_to_id));
    Ok(router)
}

async fn quests() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/", get(quest::get_all))
        .route("/", post(quest::create))
        .route("/{id}", get(quest::get_from_url))
        .route("/{id}", patch(quest::update))
        .route("/{id}/ongoing", post(quest::mark_ongoing))
        .route("/{id}/solved", post(quest::mark_solved))
        .nest("/{id}/solutions", solutions().await?);
    Ok(router)
}

async fn solutions() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/", get(solution::get_by_quest))
        .route("/", post(solution::create))
        .route("/{solution_id}", get(solution::get_from_url))
        .route("/{solution_id}/approve", post(solution::approve))
        .route("/{solution_id}/reject", post(solution::reject));
    Ok(router)
}

async fn guilds() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/", get(guild::get_all))
        .route("/", post(guild::create))
        .route("/{guildSlug}", get(guild::get_from_url))
        .route("/{guildSlug}/join", post(guild::join))
        .route("/{guildSlug}/leave", post(guild::leave));
    Ok(router)
}

async fn auth() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/github", get(auth::github_login))
        .route("/github/callback", get(auth::github_callback))
        .route("/logout", get(auth::logout));
    Ok(router)
}
