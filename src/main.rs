mod handlers;
mod plugins;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/api/downloads",
            get(handlers::downloads::get_downloads_handler)
                .post(handlers::downloads::add_downloads_handler),
        )
        .layer(CorsLayer::new());

    let addr = "127.0.0.1:4000";
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to start TCP listener");
    tracing::info!("API server listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
