use payment_tracker::{config::Config, db::Database, email::EmailFetcher};
use chrono::{Local, Duration, NaiveDate};
use std::time::Duration as StdDuration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("VIB Bank Daily Payment Tracker");
    println!("===============================\n");
    
    // Load configuration
    let config = Config::load()?;
    println!("✅ Configuration loaded");
    println!("   Email: {}", config.email.address);
    println!("   Database: {}", config.database.get_connection_string());
    
    // Initialize database
    let db = Database::new(&config.database.get_connection_string()).await?;
    println!("✅ Database connected");
    
    // Calculate date range (last 7 days)
    let today = Local::now().date_naive();
    let seven_days_ago = today - Duration::days(7);
    
    println!("\n📅 Fetching VIB bank emails from: {} to {}", seven_days_ago, today);
    
    // Create email fetcher
    let mut fetcher = EmailFetcher::new(
        &config.email.imap_server,
        config.email.imap_port,
        &config.email.address,
        &config.email.password,
    )?;
    
    // Connect to IMAP
    fetcher.connect().await?;
    println!("✅ Connected to IMAP server");
    
    // Search for VIB bank emails in date range
    let search_criteria = format!(
        "SINCE {} FROM \"vib.com.vn\" OR FROM \"vibvn.com\" OR SUBJECT \"VIB\"",
        seven_days_ago.format("%d-%b-%Y")
    );
    
    println!("\n🔍 Searching for VIB emails with criteria:");
    println!("   {}", search_criteria);
    
    let email_ids = fetcher.search_emails(&search_criteria).await?;
    println!("✅ Found {} VIB emails in date range", email_ids.len());
    
    if email_ids.is_empty() {
        println!("No VIB emails found in the last 7 days");
        return Ok(());
    }
    
    // Fetch and process each email
    println!("\n📧 Processing VIB emails:");
    let mut processed_count = 0;
    let mut saved_count = 0;
    let mut skipped_count = 0;
    
    for email_id in email_ids {
        match fetcher.fetch_email(email_id).await {
            Ok(email_text) => {
                processed_count += 1;
                
                // Parse transaction from email
                if let Some(transaction) = payment_tracker::email::parse_transaction_from_email(&email_text) {
                    println!("\n  Email #{}:", processed_count);
                    println!("    Bank: {}", transaction.bank);
                    println!("    Amount: {}", transaction.format_amount());
                    
                    // Check if transaction already exists
                    if db.transaction_exists(&transaction).await? {
                        println!("    ⚠️  Already exists in database (skipping)");
                        skipped_count += 1;
                    } else {
                        // Insert transaction with upsert support
                        match db.insert_transaction(&transaction).await {
                            Ok(id) => {
                                println!("    ✅ Saved to database (ID: {})", id);
                                saved_count += 1;
                                
                                // Show transaction details
                                if let Some(ref tid) = transaction.transaction_id {
                                    println!("    Transaction ID: {}", tid);
                                }
                                if let Some(ref emid) = transaction.email_message_id {
                                    println!("    Email Message ID: {}", emid);
                                }
                            }
                            Err(e) => {
                                println!("    ❌ Failed to save: {}", e);
                            }
                        }
                    }
                } else {
                    println!("  Email #{}: Could not parse transaction", processed_count);
                }
            }
            Err(e) => {
                println!("  ❌ Failed to fetch email {}: {}", email_id, e);
            }
        }
        
        // Small delay to avoid overwhelming the server
        tokio::time::sleep(StdDuration::from_millis(100)).await;
    }
    
    // Summary
    println!("\n📊 SUMMARY:");
    println!("  Total VIB emails found: {}", email_ids.len());
    println!("  Processed: {}", processed_count);
    println!("  Saved to database: {}", saved_count);
    println!("  Skipped (already exists): {}", skipped_count);
    
    // Show recent transactions from database
    println!("\n💾 Recent VIB transactions in database:");
    let recent_transactions = db.get_transactions(Some("out"), None, None, Some(10)).await?;
    
    for (i, transaction) in recent_transactions.iter().enumerate() {
        if transaction.bank == "VIB" {
            println!("  {}. {} - {} - {}", 
                i + 1,
                transaction.date,
                transaction.format_amount(),
                transaction.description
            );
        }
    }
    
    println!("\n🎉 Daily VIB tracking completed!");
    println!("   Database: PostgreSQL on k3s");
    println!("   Email filtering: By date range and VIB sender");
    println!("   Upsert support: Prevents duplicates");
    
    Ok(())
}