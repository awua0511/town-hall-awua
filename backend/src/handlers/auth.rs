use axum::{
    extract::{Query, State},
    response::Redirect,
};
use http::StatusCode;
use oauth2::{CsrfToken, PkceCodeChallenge, PkceCodeVerifier, Scope, TokenResponse};
use reqwest::Client;
use serde::Deserialize;
use tower_sessions::Session;

use crate::AppState;

pub async fn github_login(
    State(state): State<AppState>,
    session: Session,
) -> Result<Redirect, (StatusCode, &'static str)> {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .add_scope(Scope::new("read:user".to_string()))
        .set_pkce_challenge(pkce_challenge) // Send the PKCE challenge
        .url();

    match session.insert("csrf_token", csrf_token.secret()).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Session error: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error"));
        }
    };

    match session.insert("pkce_verifier", pkce_verifier.secret()).await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Session error: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error"));
        }
    };

    Ok(Redirect::to(auth_url.as_str()))
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,  // Temp code sent by Github, expires in 10 minutes, used to get access token
    pub state: String, // Verify if same as before, or else it's tampered / forged
}

pub async fn github_callback(
    State(state): State<AppState>,
    session: Session,
    Query(query): Query<AuthRequest>,
) -> Result<Redirect, (StatusCode, &'static str)> {
    // Check if the received CSRF token is the same as the one we sent
    let local_state: String = match session.get("csrf_token").await.unwrap() {
        Some(state) => state,
        None => {
            tracing::error!("No CSRF token found in session");
            return Err((StatusCode::FORBIDDEN, "No CSRF token found in session"));
        }
    };
    if local_state != query.state {
        return Err((StatusCode::FORBIDDEN, "Invalid CSRF token found in session"));
    }

    // Use oauth2's reqwest here
    let oauth2_http_client_result = oauth2::reqwest::ClientBuilder::new()
        // According to oauth2 docs: "Following redirects opens the client up to SSRF vulnerabilities."
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build();
    let oauth2_http_client = match oauth2_http_client_result {
        Ok(client) => client,
        Err(e) => {
            tracing::error!("Http client build error: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Http client build failed"));
        }
    };

    // Send the local PKCE verifier. This will run in the previously sent PKCE challenge on GitHub servers
    // to check if the client is still who we claim to be.
    let local_pkce_verifier = session.get("pkce_verifier").await.unwrap().unwrap();
    let to_be_sent_pkce_verifier = PkceCodeVerifier::new(local_pkce_verifier);

    // Obtain access to access token.
    let token_response_result = state
        .oauth_client
        .exchange_code(oauth2::AuthorizationCode::new(query.code))
        .set_pkce_verifier(to_be_sent_pkce_verifier)
        .request_async(&oauth2_http_client)
        .await;
    let token_response = match token_response_result {
        Ok(resp) => resp,
        Err(e) => {
            tracing::error!("Oauth Error: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Oauth token access failed"));
        }
    };

    let client = Client::new();

    let gh_user_resp_result = client
        .get("https://api.github.com/user")
        .header("User-Agent", "town-hall (by zerodrag/town-hall")
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await;
    let gh_user_resp = match gh_user_resp_result {
        Ok(resp) => resp,
        Err(e) => {
            tracing::error!("GitHub API fetch error: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "GitHub API fetch failed"));
        }
    };

    let gh_user_result = gh_user_resp.json::<serde_json::Value>().await;
    let gh_user = match gh_user_result {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("GitHub API response parse error: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "GitHub API response parse failed"));
        }
    };

    let (github_id, handle, email) = (
        gh_user["id"].as_i64().unwrap(),
        gh_user["name"].as_str().unwrap(),
        gh_user["email"].as_str().unwrap(),
    );

    let internal_user_id_result: Result<i64, _> = sqlx::query_scalar!(
        "INSERT INTO users (github_id, handle, email) \
        VALUES ($1, $2, $3) \
        ON CONFLICT (github_id) \
        DO UPDATE SET \
            handle = EXCLUDED.handle, \
            email = EXCLUDED.email \
        RETURNING user_id",
        github_id,
        handle,
        email
    )
    .fetch_one(&state.db_pool)
    .await;
    let internal_user_id = match internal_user_id_result {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Database Error: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Database Error"));
        }
    };

    let session_insert_result = session.insert("user_id", internal_user_id).await;
    match session_insert_result {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Session error: {e}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error"));
        }
    }

    Ok(Redirect::to(&format!(
        "{}/u/{handle}",
        state.frontend_url
    )))
}

pub async fn logout(State(state): State<AppState>, session: Session) -> Redirect {
    session.clear().await;
    Redirect::to(&state.frontend_url)
}
