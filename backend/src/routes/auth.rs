use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, State},
    routing::post,
    Json, Router,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use mongodb::Database;

use crate::{error::Result, models::session::Session, AppState};

pub fn create_auth_router() -> Router<AppState> {
    Router::new().route("/session", post(post_session))
}

#[derive(Debug, serde::Deserialize)]
struct PostSessionRequest {
    pub username: String,
}

#[derive(Debug, serde::Serialize)]
struct PostSessionResponse {
    pub username: String,
}

async fn post_session(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    State(db): State<Database>,
    Json(signup_request): Json<PostSessionRequest>,
) -> Result<Json<PostSessionResponse>> {
    tracing::debug!("Received session request: {:?}", signup_request);

    let session = Session::new(
        &db,
        &signup_request.username,
        &addr.ip().to_string(),
        &user_agent.to_string(),
    )
    .await?;

    tracing::debug!("Created session for username {}", session.username);

    Ok(Json(PostSessionResponse {
        username: session.username,
    }))
}
