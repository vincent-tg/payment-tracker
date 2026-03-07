use axum::{Router, http::StatusCode, response::Json, routing::get};
use serde_json::json;
use std::net::SocketAddr;

pub async fn start_health_server(port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(root));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🌐 Health server starting on http://0.0.0.0:{}", port);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    let response = json!({
        "status": "healthy",
        "service": "vib-payment-tracker",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "features": ["vib-bank", "currency-tracking", "postgresql", "email-parsing"]
    });

    (StatusCode::OK, Json(response))
}

async fn root() -> (StatusCode, Json<serde_json::Value>) {
    let response = json!({
        "message": "VIB Bank Payment Tracker API",
        "endpoints": {
            "health": "/health",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Specialized for VIB bank transactions with currency tracking"
        }
    });

    (StatusCode::OK, Json(response))
}
