use payment_tracker::db::Database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Initializing PostgreSQL database for payment tracker...");

    // Use the connection string from config.toml
    let connection_string =
        "postgres://payment_user:payment_password@10.0.0.229:30432/payment_tracker";

    println!("Connecting to: {}", connection_string);

    // Initialize database
    Database::init_database(connection_string).await?;

    println!("✅ Database initialized successfully!");

    // Test connection
    let db = Database::new(connection_string).await?;
    println!("✅ Database connection test passed!");

    // Create a test transaction
    use chrono::NaiveDate;
    use payment_tracker::models::Transaction;

    let test_transaction = Transaction {
        id: 0,
        date: NaiveDate::from_ymd_opt(2026, 3, 6).unwrap(),
        description: "Test VIB Transaction".to_string(),
        amount: 58000.0,
        currency: "VND".to_string(),
        r#type: "out".to_string(),
        source: "test".to_string(),
        bank: "VIB".to_string(),
        transaction_id: None,
        email_message_id: None,
        created_at: chrono::Local::now(),
    };

    println!("\nTest Transaction:");
    println!("  Bank: {}", test_transaction.bank);
    println!("  Amount: {}", test_transaction.format_amount());
    println!(
        "  With conversion: {}",
        test_transaction.format_with_conversion()
    );

    // Try to insert
    match db.insert_transaction(&test_transaction).await {
        Ok(id) => println!("✅ Test transaction inserted with ID: {}", id),
        Err(e) => println!("⚠️  Could not insert (might be duplicate): {}", e),
    }

    println!("\n🎉 PostgreSQL database is ready for VIB bank transactions!");
    Ok(())
}
