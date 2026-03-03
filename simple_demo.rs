// Simple demonstration of the payment tracker library
use payment_tracker::{config::Config, db, email, App};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Payment Tracker Demo ===\n");
    
    // Clean up any existing test database
    let test_db = "demo_payments.db";
    if std::path::Path::new(test_db).exists() {
        fs::remove_file(test_db)?;
    }
    
    // Create a simple config
    let mut config = Config::default();
    config.database.path = test_db.to_string();
    
    // Initialize database
    println!("1. Initializing database...");
    tokio::runtime::Runtime::new()?.block_on(async {
        db::Database::init_database(&config.database.path).await?;
        println!("   Database created: {}", config.database.path);
        
        // Create app instance
        println!("\n2. Creating application instance...");
        let app = App::new(config).await?;
        
        // Add some sample transactions
        println!("\n3. Adding sample transactions...");
        
        // Income
        app.add_manual_transaction(1000.00, "Salary", "in", Some("2024-01-15")).await?;
        println!("   Added: Salary - $1000.00 (in)");
        
        // Expenses
        app.add_manual_transaction(50.00, "Groceries", "out", Some("2024-01-16")).await?;
        println!("   Added: Groceries - $50.00 (out)");
        
        app.add_manual_transaction(25.00, "Coffee", "out", Some("2024-01-17")).await?;
        println!("   Added: Coffee - $25.00 (out)");
        
        app.add_manual_transaction(100.00, "Electricity Bill", "out", Some("2024-01-18")).await?;
        println!("   Added: Electricity Bill - $100.00 (out)");
        
        app.add_manual_transaction(200.00, "Freelance Work", "in", Some("2024-01-19")).await?;
        println!("   Added: Freelance Work - $200.00 (in)");
        
        // List transactions
        println!("\n4. Listing all transactions:");
        app.list_transactions(None, None, None, None).await?;
        
        // Generate summary
        println!("\n5. Generating monthly summary:");
        app.generate_summary("month", None).await?;
        
        // Test email parsing
        println!("\n6. Testing email parsing...");
        let example_email = r#"Subject: Transaction Alert
From: yourbank@example.com
Date: 20/01/2024

Dear Customer,

A transaction has been made on your account:

Amount: $75.50
Description: RESTAURANT DINNER
Date: 20/01/2024
Transaction Type: DEBITED

Thank you,
Your Bank"#;
        
        if let Some(transaction) = email::parse_transaction_from_email(example_email) {
            println!("   Parsed from email:");
            println!("   - Date: {}", transaction.date.format("%Y-%m-%d"));
            println!("   - Description: {}", transaction.description);
            println!("   - Amount: ${:.2}", transaction.amount);
            println!("   - Type: {}", transaction.r#type);
        } else {
            println!("   Could not parse transaction from email");
        }
        
        println!("\n=== Demo Complete ===");
        println!("Database file: {}", test_db);
        println!("You can explore the database using:");
        println!("  sqlite3 {} '.schema'", test_db);
        println!("  sqlite3 {} 'SELECT * FROM transactions;'", test_db);
        
        Ok::<(), anyhow::Error>(())
    })?;
    
    Ok(())
}