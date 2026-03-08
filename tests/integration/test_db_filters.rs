use chrono::Local;
use payment_tracker::{db::Database, models::Transaction};

fn test_db_url() -> Option<String> {
    std::env::var("DATABASE_URL")
        .or_else(|_| std::env::var("SUPABASE_CONNECTION_STRING"))
        .ok()
        .filter(|v| !v.trim().is_empty())
}

#[tokio::test]
async fn test_get_transactions_filters_postgres() {
    let Some(db_url) = test_db_url() else {
        eprintln!("Skipping integration test: DATABASE_URL/SUPABASE_CONNECTION_STRING not set");
        return;
    };

    Database::init_database(&db_url)
        .await
        .expect("database init should succeed");

    let db = Database::new(&db_url)
        .await
        .expect("database connection should succeed");

    let today = Local::now().date_naive();

    let tx_out = Transaction::from_email(
        today,
        "Coffee Shop".to_string(),
        5.5,
        "USD".to_string(),
        "out".to_string(),
        "VIB".to_string(),
        Some(format!("test-out-{}", Local::now().timestamp_nanos_opt().unwrap_or(0))),
        None,
    );

    let tx_in = Transaction::from_email(
        today,
        "Salary".to_string(),
        100.0,
        "USD".to_string(),
        "in".to_string(),
        "VIB".to_string(),
        Some(format!("test-in-{}", Local::now().timestamp_nanos_opt().unwrap_or(0))),
        None,
    );

    db.insert_transaction(&tx_out)
        .await
        .expect("insert out transaction should succeed");
    db.insert_transaction(&tx_in)
        .await
        .expect("insert in transaction should succeed");

    let out_rows = db
        .get_transactions(Some("out"), None, None, Some(20))
        .await
        .expect("query out rows should succeed");

    assert!(out_rows.iter().any(|t| t.description == "Coffee Shop"));
    assert!(out_rows.iter().all(|t| t.r#type == "out"));

    let today_str = today.format("%Y-%m-%d").to_string();
    let day_rows = db
        .get_transactions(None, Some(&today_str), Some(&today_str), Some(50))
        .await
        .expect("date range query should succeed");

    assert!(day_rows.iter().any(|t| t.description == "Coffee Shop"));
    assert!(day_rows.iter().any(|t| t.description == "Salary"));
}
