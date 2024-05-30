use std::{net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};

use axum::{
    extract::{FromRef, Request, State},
    http::{Method, StatusCode, Uri},
    Json, Router, ServiceExt,
};
use cards::CardSet;
use clap::Parser;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use serde_json::{json, Value};
use tower_http::{
    cors::{Any, CorsLayer},
    normalize_path::NormalizePath,
    trace::TraceLayer,
};

use crate::{cards::load_card_data, routes::create_router};

mod cards;
mod error;
mod models;
mod routes;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, default_value = "3000")]
    pub port: u16,

    #[arg(long, default_value = "mongodb://127.0.0.1:27017")]
    pub database_uri: String,

    #[arg(long, default_value = "cah_online")]
    pub database_name: String,

    #[arg(long)]
    pub cards_path: PathBuf,
}

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub cards: Arc<[CardSet]>,
}

impl FromRef<AppState> for Database {
    fn from_ref(app_state: &AppState) -> Database {
        app_state.database.clone()
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    /* Initialize App State */

    let database = init_db(&args.database_uri, &args.database_name)
        .await
        .expect("Failed to initialize DB");

    let cards = load_card_data(&args.cards_path)?.into();

    let state = AppState { database, cards };

    /* CORS Support */

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH])
        .allow_origin(Any);

    /* Initialize Application */

    let app = NormalizePath::trim_trailing_slash(
        Router::new()
            .fallback(fallback)
            .nest("/api", create_router(state.clone()))
            .layer(cors)
            .layer(TraceLayer::new_for_http())
            .with_state(state),
    );

    /* Serve our app this hyper */

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Listening on: http://{}", addr);

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app)).await
}

async fn init_db(database_uri: &str, database_name: &str) -> mongodb::error::Result<Database> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse(database_uri).await?;

    // Set a low timeout for connecting to the database
    client_options.connect_timeout = Some(Duration::from_millis(2000));
    client_options.server_selection_timeout = Some(Duration::from_millis(2000));

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // Send a ping to ensure a successful connection
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    tracing::debug!("Successfully connected to MongoDB!");

    // Get a handle to a database.
    Ok(client.database(database_name))
}

/// Generic 404 fallback handler
async fn fallback(uri: Uri) -> (StatusCode, Json<Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "NOT_FOUND",
            "message": format!("The requested resource `{}` was not found", uri)
        })),
    )
}

async fn serve_frontend(
    State(client): State<Client>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let uri = format!("http://127.0.0.1:3000{}", path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
