use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::currency::CurrencyConverter;
use crate::db::Database;
use crate::email::{self, EmailClient};
use crate::models::Transaction;

// ---------------------------------------------------------------------------
// Shared application state
// ---------------------------------------------------------------------------

pub struct AppState {
    pub db: Database,
    pub config: Config,
    pub currency: CurrencyConverter,
}

type SharedState = Arc<AppState>;

// ---------------------------------------------------------------------------
// API response helpers
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    fn ok(data: T) -> Json<ApiResponse<T>> {
        Json(ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        })
    }
}

fn api_error(status: StatusCode, msg: impl Into<String>) -> impl IntoResponse {
    let body = json!({
        "success": false,
        "data": null,
        "error": msg.into()
    });
    (status, Json(body))
}

// ---------------------------------------------------------------------------
// Request / response DTOs
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct TransactionQuery {
    pub r#type: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub limit: Option<i64>,
    pub bank: Option<String>,
    pub currency: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub amount: f64,
    pub description: String,
    pub r#type: String, // "in" or "out"
    pub date: Option<String>,
    pub currency: Option<String>,
    pub bank: Option<String>,
}

#[derive(Deserialize)]
pub struct SummaryQuery {
    pub period: Option<String>, // day, week, month, year
    pub date: Option<String>,
}

#[derive(Deserialize)]
pub struct ConvertCurrencyRequest {
    pub amount: f64,
    pub from: String,
    pub to: String,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub id: i64,
    pub date: String,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub r#type: String,
    pub source: String,
    pub bank: String,
    pub amount_usd: f64,
    pub formatted_amount: String,
    pub transaction_id: Option<String>,
    pub email_message_id: Option<String>,
    pub created_at: String,
}

impl From<Transaction> for TransactionResponse {
    fn from(t: Transaction) -> Self {
        Self {
            id: t.id,
            date: t.date.to_string(),
            description: t.description.clone(),
            amount: t.amount,
            currency: t.currency.clone(),
            r#type: t.r#type.clone(),
            source: t.source.clone(),
            bank: t.bank.clone(),
            amount_usd: t.to_usd(),
            formatted_amount: t.format_with_conversion(),
            transaction_id: t.transaction_id.clone(),
            email_message_id: t.email_message_id.clone(),
            created_at: t.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct SummaryResponse {
    pub period: String,
    pub total_transactions: i64,
    pub total_in: f64,
    pub total_out: f64,
    pub net_balance: f64,
    pub top_categories: Vec<CategoryAmount>,
}

#[derive(Serialize)]
pub struct CategoryAmount {
    pub category: String,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct FetchResult {
    pub emails_found: usize,
    pub parsed: usize,
    pub saved: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

#[derive(Serialize)]
pub struct ConvertResult {
    pub from_amount: f64,
    pub from_currency: String,
    pub to_amount: f64,
    pub to_currency: String,
}

#[derive(Serialize)]
pub struct CurrencyInfo {
    pub currencies: Vec<String>,
}

// ---------------------------------------------------------------------------
// Router setup
// ---------------------------------------------------------------------------

pub async fn start_api_server(port: u16, state: SharedState) -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Health / info
        .route("/", get(root))
        .route("/health", get(health_check))
        // Transactions CRUD
        .route("/api/transactions", get(list_transactions))
        .route("/api/transactions", post(create_transaction))
        .route("/api/transactions/{id}", get(get_transaction))
        .route("/api/transactions/{id}", delete(delete_transaction))
        // Email fetch
        .route("/api/fetch", post(fetch_emails))
        // Analytics
        .route("/api/summary", get(get_summary))
        // Currency
        .route("/api/currencies", get(list_currencies))
        .route("/api/currencies/convert", post(convert_currency))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("🚀 Payment Tracker API starting on http://0.0.0.0:{}", port);
    println!("🚀 Payment Tracker API on http://0.0.0.0:{}", port);
    println!();
    println!("  Endpoints:");
    println!("    GET    /health                    Health check");
    println!("    GET    /api/transactions           List transactions");
    println!("    POST   /api/transactions           Create transaction");
    println!("    GET    /api/transactions/:id        Get transaction");
    println!("    DELETE /api/transactions/:id        Delete transaction");
    println!("    POST   /api/fetch                  Fetch & process emails");
    println!("    GET    /api/summary                Transaction summary");
    println!("    GET    /api/currencies              List currencies");
    println!("    POST   /api/currencies/convert      Convert currency");

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

// Backwards-compatible simple health server (kept for CLI `serve` command)
pub async fn start_health_server(port: u16) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/health", get(health_check_simple))
        .route("/", get(root_simple));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🌐 Health server starting on http://0.0.0.0:{}", port);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Health / root handlers
// ---------------------------------------------------------------------------

async fn health_check(State(state): State<SharedState>) -> impl IntoResponse {
    // Basic DB connectivity test – SELECT 1 via ping()
    let db_status = match state.db.ping().await {
        Ok(_) => "ok",
        Err(e) => {
            tracing::error!(error = %e, "Database health check failed");
            "error"
        }
    };

    let response = json!({
        "status": if db_status == "ok" { "healthy" } else { "degraded" },
        "service": "payment-tracker",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
        "api": "v1",
        "features": ["transactions", "email-parsing", "currency-conversion", "postgresql"],
        "db": db_status,
    });
    (StatusCode::OK, Json(response))
}

async fn health_check_simple() -> impl IntoResponse {
    let response = json!({
        "status": "healthy",
        "service": "payment-tracker",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION"),
    });
    (StatusCode::OK, Json(response))
}

async fn root(State(_state): State<SharedState>) -> impl IntoResponse {
    let response = json!({
        "service": "VIB Payment Tracker API",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "health":                "GET  /health",
            "list_transactions":     "GET  /api/transactions?type=in&from=2026-01-01&to=2026-12-31&limit=50",
            "create_transaction":    "POST /api/transactions",
            "get_transaction":       "GET  /api/transactions/:id",
            "delete_transaction":    "DELETE /api/transactions/:id",
            "fetch_emails":          "POST /api/fetch",
            "summary":               "GET  /api/summary?period=month&date=2026-03-01",
            "list_currencies":       "GET  /api/currencies",
            "convert_currency":      "POST /api/currencies/convert"
        }
    });
    (StatusCode::OK, Json(response))
}

async fn root_simple() -> impl IntoResponse {
    let response = json!({
        "service": "VIB Payment Tracker",
        "version": env!("CARGO_PKG_VERSION"),
    });
    (StatusCode::OK, Json(response))
}

// ---------------------------------------------------------------------------
// Transaction handlers
// ---------------------------------------------------------------------------

/// GET /api/transactions?type=out&from=2026-01-01&to=2026-03-31&limit=50
async fn list_transactions(
    State(state): State<SharedState>,
    Query(params): Query<TransactionQuery>,
) -> impl IntoResponse {
    match state
        .db
        .get_transactions(
            params.r#type.as_deref(),
            params.from.as_deref(),
            params.to.as_deref(),
            params.limit,
        )
        .await
    {
        Ok(transactions) => {
            let response: Vec<TransactionResponse> =
                transactions.into_iter().map(|t| t.into()).collect();
            ApiResponse::ok(response).into_response()
        }
        Err(e) => api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch transactions: {}", e),
        )
        .into_response(),
    }
}

/// POST /api/transactions
async fn create_transaction(
    State(state): State<SharedState>,
    Json(body): Json<CreateTransactionRequest>,
) -> impl IntoResponse {
    // Validate type
    if body.r#type != "in" && body.r#type != "out" {
        return api_error(StatusCode::BAD_REQUEST, "type must be 'in' or 'out'").into_response();
    }

    if body.amount <= 0.0 {
        return api_error(StatusCode::BAD_REQUEST, "amount must be positive").into_response();
    }

    if body.description.trim().is_empty() {
        return api_error(StatusCode::BAD_REQUEST, "description cannot be empty").into_response();
    }

    let mut transaction = Transaction::new_manual(
        body.amount,
        &body.description,
        &body.r#type,
        body.date.as_deref(),
    );

    // Override defaults if provided
    if let Some(currency) = &body.currency {
        transaction.currency = currency.to_uppercase();
    }
    if let Some(bank) = &body.bank {
        transaction.bank = bank.clone();
    }

    match state.db.insert_transaction(&transaction).await {
        Ok(id) => {
            transaction.id = id;
            let response: TransactionResponse = transaction.into();
            (StatusCode::CREATED, ApiResponse::ok(response)).into_response()
        }
        Err(e) => api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create transaction: {}", e),
        )
        .into_response(),
    }
}

/// GET /api/transactions/:id
async fn get_transaction(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match state.db.get_transaction_by_id(id).await {
        Ok(Some(transaction)) => {
            let response: TransactionResponse = transaction.into();
            ApiResponse::ok(response).into_response()
        }
        Ok(None) => api_error(
            StatusCode::NOT_FOUND,
            format!("Transaction {} not found", id),
        )
        .into_response(),
        Err(e) => api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch transaction: {}", e),
        )
        .into_response(),
    }
}

/// DELETE /api/transactions/:id
async fn delete_transaction(
    State(state): State<SharedState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match state.db.delete_transaction(id).await {
        Ok(true) => {
            let response = json!({ "deleted": true, "id": id });
            ApiResponse::ok(response).into_response()
        }
        Ok(false) => api_error(
            StatusCode::NOT_FOUND,
            format!("Transaction {} not found", id),
        )
        .into_response(),
        Err(e) => api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete transaction: {}", e),
        )
        .into_response(),
    }
}

// ---------------------------------------------------------------------------
// Email fetch handler
// ---------------------------------------------------------------------------

/// POST /api/fetch — trigger email fetch and processing
async fn fetch_emails(State(state): State<SharedState>) -> impl IntoResponse {
    let config = &state.config;

    // Validate email config
    if config.email.address.is_empty() || config.email.password.is_empty() {
        return api_error(
            StatusCode::BAD_REQUEST,
            "Email not configured. Set email address and password in config.toml",
        )
        .into_response();
    }

    let email_client = match EmailClient::new(
        &config.email.imap_server,
        config.email.imap_port,
        &config.email.address,
        &config.email.password,
    ) {
        Ok(client) => client,
        Err(e) => {
            return api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create email client: {}", e),
            )
            .into_response();
        }
    };

    let emails = match email_client.fetch_recent_emails().await {
        Ok(emails) => emails,
        Err(e) => {
            return api_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to fetch emails: {}", e),
            )
            .into_response();
        }
    };

    let mut result = FetchResult {
        emails_found: emails.len(),
        parsed: 0,
        saved: 0,
        skipped: 0,
        errors: Vec::new(),
    };

    for email_text in &emails {
        if let Some(transaction) = email::parse_transaction_from_email(email_text) {
            result.parsed += 1;

            match state.db.transaction_exists(&transaction).await {
                Ok(true) => {
                    result.skipped += 1;
                }
                Ok(false) => match state.db.insert_transaction(&transaction).await {
                    Ok(_) => {
                        result.saved += 1;
                    }
                    Err(e) => {
                        result.errors.push(format!("Insert failed: {}", e));
                    }
                },
                Err(e) => {
                    result.errors.push(format!("Existence check failed: {}", e));
                }
            }
        }
    }

    ApiResponse::ok(result).into_response()
}

// ---------------------------------------------------------------------------
// Summary handler
// ---------------------------------------------------------------------------

/// GET /api/summary?period=month&date=2026-03-01
async fn get_summary(
    State(state): State<SharedState>,
    Query(params): Query<SummaryQuery>,
) -> impl IntoResponse {
    let period = params.period.as_deref().unwrap_or("month");

    match state.db.get_summary(period, params.date.as_deref()).await {
        Ok(summary) => {
            let response = SummaryResponse {
                period: period.to_string(),
                total_transactions: summary.total_transactions,
                total_in: summary.total_in,
                total_out: summary.total_out,
                net_balance: summary.net_balance,
                top_categories: summary
                    .top_categories
                    .into_iter()
                    .map(|(category, amount)| CategoryAmount { category, amount })
                    .collect(),
            };
            ApiResponse::ok(response).into_response()
        }
        Err(e) => api_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get summary: {}", e),
        )
        .into_response(),
    }
}

// ---------------------------------------------------------------------------
// Currency handlers
// ---------------------------------------------------------------------------

/// GET /api/currencies
async fn list_currencies(State(state): State<SharedState>) -> impl IntoResponse {
    let currencies = state.currency.get_supported_currencies();
    let mut sorted: Vec<String> = currencies.into_iter().map(|s| s.to_string()).collect();
    sorted.sort();
    ApiResponse::ok(CurrencyInfo { currencies: sorted })
}

/// POST /api/currencies/convert
async fn convert_currency(
    State(state): State<SharedState>,
    Json(body): Json<ConvertCurrencyRequest>,
) -> impl IntoResponse {
    if body.amount <= 0.0 {
        return api_error(StatusCode::BAD_REQUEST, "amount must be positive").into_response();
    }

    match state.currency.convert(body.amount, &body.from, &body.to) {
        Ok(result) => ApiResponse::ok(ConvertResult {
            from_amount: body.amount,
            from_currency: body.from.to_uppercase(),
            to_amount: result,
            to_currency: body.to.to_uppercase(),
        })
        .into_response(),
        Err(e) => {
            api_error(StatusCode::BAD_REQUEST, format!("Conversion failed: {}", e)).into_response()
        }
    }
}
