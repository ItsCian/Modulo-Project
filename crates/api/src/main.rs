use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let app = Router::new().route("/health", get(|| async { "ok" }));

    // 0.0.0.0 so other containers (and your browser through the port mapping) can reach it
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("failed to bind the port");

    println!("API listening on port {port}");
    axum::serve(listener, app).await.expect("server error");
}
