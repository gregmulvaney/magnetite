mod entities;
mod handlers;
mod plugins;

use axum::{routing::get, Router};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    // Initialize tracing logger
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Initialize database connection
    let dsn = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut db_conn_opts = ConnectOptions::new(&dsn);
    db_conn_opts.sqlx_logging(false);
    let db_conn = Database::connect(db_conn_opts).await.unwrap();

    // Initialize app state
    let state = AppState { db: db_conn };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/api/downloads",
            get(handlers::downloads::get_downloads_handler)
                .post(handlers::downloads::add_downloads_handler),
        )
        .layer(CorsLayer::new())
        .with_state(state);

    let addr = "127.0.0.1:4000";
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to start TCP listener");
    tracing::info!("API server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
