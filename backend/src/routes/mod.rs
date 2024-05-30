use axum::{routing::get, Json, Router};
use serde_json::json;

use crate::AppState;

use self::{auth::create_auth_router, cards::create_cards_router};

mod auth;
mod cards;

pub fn create_router(_state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(|| async {
                Json(json!({
                    "message": "Welcome to the CAH Online API",
                    "version": "0.1.0"
                }))
            }),
        )
        .nest("/auth", create_auth_router())
        .nest("/cards", create_cards_router())
}
