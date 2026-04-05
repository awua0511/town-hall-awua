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
pub struct Solution {
    #[serde_as(as = "DisplayFromStr")]
    solution_id: i64,
    #[serde_as(as = "DisplayFromStr")]
    quest_id: i64,
    #[serde_as(as = "DisplayFromStr")]
    adventurer_id: i64,
    github_link: String,
    description: Option<String>,
    status: String,
    #[serde(with = "time::serde::rfc3339")]
    created_at: time::OffsetDateTime,
}

#[derive(Deserialize, specta::Type)]
pub struct CreateSolutionRequest {
    quest_id: i64,
    github_link: String,
    description: Option<String>,
}

#[axum::debug_handler]
pub async fn get_from_url(
    Path(solution_id): Path<i64>,
    State(state): State<AppState>,
) -> SimpResp<Json<Solution>> {
    let result = query_as!(
        Solution,
        "SELECT solution_id, quest_id, adventurer_id, github_link, description, status, created_at \
        FROM solutions \
        WHERE solution_id=$1",
        solution_id
    )
    .fetch_one(&state.db_pool)
    .await;
    match result {
        Ok(solution) => Ok(Json(solution)),
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "Solution ID not found")),
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}

#[axum::debug_handler]
pub async fn get_by_quest(
    Path(quest_id): Path<i64>,
    State(state): State<AppState>,
) -> SimpResp<Json<Vec<Solution>>> {
    let result = query_as!(
        Solution,
        "SELECT solution_id, quest_id, adventurer_id, github_link, description, status, created_at \
        FROM solutions \
        WHERE quest_id=$1 \
        ORDER BY created_at DESC",
        quest_id
    )
    .fetch_all(&state.db_pool)
    .await;
    match result {
        Ok(solutions) => Ok(Json(solutions)),
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
    Json(req): Json<CreateSolutionRequest>,
) -> SimpResp<Json<i64>> {
    let user_id = helper::resolve_current_user_id(&session).await?;
    
    // Check if the quest exists
    let quest_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM quests WHERE quest_id=$1)",
        req.quest_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database Error: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
    })?;
    
    if !quest_exists {
        return Err((StatusCode::NOT_FOUND, "Quest ID not found"));
    }
    
    // Create the solution
    let result: Result<i64, _> = sqlx::query_scalar!(
        "INSERT INTO solutions (quest_id, adventurer_id, github_link, description) \
        VALUES ($1, $2, $3, $4) \
        RETURNING solution_id",
        req.quest_id,
        user_id,
        req.github_link,
        req.description
    )
    .fetch_one(&state.db_pool)
    .await;
    
    match result {
        Ok(solution_id) => Ok(Json(solution_id)),
        Err(e) => {
            tracing::error!("Database Error: {e}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"))
        }
    }
}

#[axum::debug_handler]
pub async fn approve(
    session: Session,
    Path(solution_id): Path<i64>,
    State(state): State<AppState>,
) -> SimpResp<Json<()>> {
    let user_id = helper::resolve_current_user_id(&session).await?;
    
    // Check if the solution exists and get the quest_id
    let solution = sqlx::query!(
        "SELECT quest_id FROM solutions WHERE solution_id=$1",
        solution_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        if e == sqlx::Error::RowNotFound {
            (StatusCode::NOT_FOUND, "Solution ID not found")
        } else {
            tracing::error!("Database Error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
        }
    })?;
    
    // Check if the user is the poster of the quest
    let quest_poster_id = sqlx::query_scalar!(
        "SELECT poster_id FROM quests WHERE quest_id=$1",
        solution.quest_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database Error: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
    })?;
    
    if quest_poster_id != user_id {
        return Err((StatusCode::FORBIDDEN, "You are not the poster of this quest"));
    }
    
    // Approve the solution
    let result = sqlx::query!(
        "UPDATE solutions \
        SET status = 'approved' \
        WHERE solution_id=$1",
        solution_id
    )
    .execute(&state.db_pool)
    .await;
    
    match result {
        Ok(_) => {
            // Mark the quest as solved
            let quest_result = sqlx::query!(
                "UPDATE quests \
                SET status = 'solved' \
                WHERE quest_id=$1",
                solution.quest_id
            )
            .execute(&state.db_pool)
            .await;
            
            match quest_result {
                Ok(_) => Ok(Json(())),
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
pub async fn reject(
    session: Session,
    Path(solution_id): Path<i64>,
    State(state): State<AppState>,
) -> SimpResp<Json<()>> {
    let user_id = helper::resolve_current_user_id(&session).await?;
    
    // Check if the solution exists and get the quest_id
    let solution = sqlx::query!(
        "SELECT quest_id FROM solutions WHERE solution_id=$1",
        solution_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        if e == sqlx::Error::RowNotFound {
            (StatusCode::NOT_FOUND, "Solution ID not found")
        } else {
            tracing::error!("Database Error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
        }
    })?;
    
    // Check if the user is the poster of the quest
    let quest_poster_id = sqlx::query_scalar!(
        "SELECT poster_id FROM quests WHERE quest_id=$1",
        solution.quest_id
    )
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Database Error: {e}");
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
    })?;
    
    if quest_poster_id != user_id {
        return Err((StatusCode::FORBIDDEN, "You are not the poster of this quest"));
    }
    
    // Reject the solution
    let result = sqlx::query!(
        "UPDATE solutions \
        SET status = 'rejected' \
        WHERE solution_id=$1",
        solution_id
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