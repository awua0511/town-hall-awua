use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};
use sqlx::query_as;
use tower_sessions::Session;

use crate::{
    AppState,
    handlers::helper::{self, SimpResp},
};

#[serde_as]
#[derive(Serialize, specta::Type)]
pub struct Guild {
    #[serde_as(as = "DisplayFromStr")]
    guild_id: i64,
    name: String,
    slug: String,
    description: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    created_at: time::OffsetDateTime,
}

#[derive(Deserialize, specta::Type)]
pub struct CreateGuildRequest {
    name: String,
    description: Option<String>,
}

#[axum::debug_handler]
pub async fn get_from_url(
    Path(guild_slug): Path<String>,
    State(state): State<AppState>,
) -> SimpResp<Json<Guild>> {
    let result = query_as!(
        Guild,
        "SELECT guild_id, name, slug, description, created_at \
        FROM guilds \
        WHERE slug=$1",
        guild_slug
    )
    .fetch_one(&state.db_pool)
    .await;
    match result {
        Ok(guild) => Ok(Json(guild)),
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "Guild not found")),
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}

#[axum::debug_handler]
pub async fn get_all(State(state): State<AppState>) -> SimpResp<Json<Vec<Guild>>> {
    let result = query_as!(
        Guild,
        "SELECT guild_id, name, slug, description, created_at \
        FROM guilds \
        ORDER BY created_at DESC"
    )
    .fetch_all(&state.db_pool)
    .await;
    match result {
        Ok(guilds) => Ok(Json(guilds)),
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}

#[axum::debug_handler]
pub async fn create(
    session: Session,
    State(state): State<AppState>,
    Json(req): Json<CreateGuildRequest>,
) -> SimpResp<Json<i64>> {
    let user_id = helper::resolve_current_user_id(&session).await?;
    
    // Generate slug from name
    let slug = req.name
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "-")
        .replace("-", "-")
        .trim_matches('-')
        .to_string();
    
    // Create the guild
    let result: Result<i64, _> = sqlx::query_scalar!(
        "INSERT INTO guilds (name, slug, description) \
        VALUES ($1, $2, $3) \
        RETURNING guild_id",
        req.name,
        slug,
        req.description
    )
    .fetch_one(&state.db_pool)
    .await;
    
    match result {
        Ok(guild_id) => {
            // Add the creator as owner
            let member_result = sqlx::query!(
                "INSERT INTO guild_members (guild_id, user_id, role) \
                VALUES ($1, $2, 'owner')",
                guild_id,
                user_id
            )
            .execute(&state.db_pool)
            .await;
            
            match member_result {
                Ok(_) => Ok(Json(guild_id)),
                Err(e) => {
                    tracing::error!("Database Error: {e}");
                    Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
                }
            }
        }
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}

#[axum::debug_handler]
pub async fn join(
    session: Session,
    Path(guild_slug): Path<String>,
    State(state): State<AppState>,
) -> SimpResp<Json<()>> {
    let user_id = helper::resolve_current_user_id(&session).await?;
    
    // Get guild_id from slug
    let guild_id = sqlx::query_scalar!(
        "SELECT guild_id FROM guilds WHERE slug=$1",
        guild_slug
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        if e == sqlx::Error::RowNotFound {
            (StatusCode::NOT_FOUND, "Guild not found")
        } else {
            tracing::error!("Database Error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
        }
    })?;
    
    // Check if user is already a member
    let is_member = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM guild_members WHERE guild_id=$1 AND user_id=$2)",
        guild_id,
        user_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database Error: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
    })?;
    
    if is_member {
        return Err((StatusCode::BAD_REQUEST, "You are already a member of this guild"));
    }
    
    // Add user as member
    let result = sqlx::query!(
        "INSERT INTO guild_members (guild_id, user_id, role) \
        VALUES ($1, $2, 'member')",
        guild_id,
        user_id
    )
    .execute(&state.db_pool)
    .await;
    
    match result {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}

#[axum::debug_handler]
pub async fn leave(
    session: Session,
    Path(guild_slug): Path<String>,
    State(state): State<AppState>,
) -> SimpResp<Json<()>> {
    let user_id = helper::resolve_current_user_id(&session).await?;
    
    // Get guild_id from slug
    let guild_id = sqlx::query_scalar!(
        "SELECT guild_id FROM guilds WHERE slug=$1",
        guild_slug
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        if e == sqlx::Error::RowNotFound {
            (StatusCode::NOT_FOUND, "Guild not found")
        } else {
            tracing::error!("Database Error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
        }
    })?;
    
    // Check if user is a member
    let role = sqlx::query_scalar!(
        "SELECT role FROM guild_members WHERE guild_id=$1 AND user_id=$2",
        guild_id,
        user_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        if e == sqlx::Error::RowNotFound {
            (StatusCode::BAD_REQUEST, "You are not a member of this guild")
        } else {
            tracing::error!("Database Error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
        }
    })?;
    
    // Owner cannot leave the guild
    if role == "owner" {
        return Err((StatusCode::BAD_REQUEST, "Owner cannot leave the guild"));
    }
    
    // Remove user from guild
    let result = sqlx::query!(
        "DELETE FROM guild_members WHERE guild_id=$1 AND user_id=$2",
        guild_id,
        user_id
    )
    .execute(&state.db_pool)
    .await;
    
    match result {
        Ok(_) => Ok(Json(())),
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}