use std::sync::Arc;

use axum::{extract::State, routing::get, Json, Router};

use crate::{cards::CardSet, AppState};

pub fn create_cards_router() -> Router<AppState> {
    Router::new().route("/", get(get_cards))
}

async fn get_cards(State(state): State<AppState>) -> Json<Arc<[CardSet]>> {
    Json(state.cards)
}
