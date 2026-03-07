use chrono::{Duration, Local};
use payment_tracker::{config::Config, db::Database, email::EmailClient};

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

    println!(
        "\n📅 Fetching VIB bank emails from: {} to {}",
        seven_days_ago, today
    );

    // Create email client
    let client = EmailClient::new(
        &config.email.imap_server,
        config.email.imap_port,
        &config.email.address,
        &config.email.password,
    )?;

    // Fetch recent emails
    println!("\n📧 Fetching emails from IMAP server...");
    let emails = client.fetch_recent_emails().await?;

    if emails.is_empty() {
        println!("No emails found in the last 7 days");
        return Ok(());
    }

    println!("Found {} emails", emails.len());

    // Process each email
    println!("\n📧 Processing emails:");
    let mut processed_count = 0;
    let mut saved_count = 0;
    let mut skipped_count = 0;

    for (i, email_text) in emails.iter().enumerate() {
        processed_count += 1;

        // Parse transaction from email
        if let Some(transaction) = payment_tracker::email::parse_transaction_from_email(email_text)
        {
            println!("\n  Email #{}:", i + 1);
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
                    }
                    Err(e) => {
                        println!("    ❌ Failed to save: {}", e);
                    }
                }
            }
        } else {
            println!(
                "  Email #{}: No transaction found (not a VIB email or different format)",
                i + 1
            );
        }
    }

    // Summary
    println!("\n📊 Summary:");
    println!("  Total emails processed: {}", processed_count);
    println!("  Transactions saved: {}", saved_count);
    println!("  Transactions skipped (already exist): {}", skipped_count);

    // Show recent transactions from database
    println!("\n💾 Recent transactions in database:");
    let recent_transactions = db
        .get_transactions(
            None,                              // type filter
            Some(&seven_days_ago.to_string()), // from date
            Some(&today.to_string()),          // to date
            Some(100),                         // limit
        )
        .await?;

    if recent_transactions.is_empty() {
        println!("  No transactions found in the date range");
    } else {
        for (i, tx) in recent_transactions.iter().enumerate() {
            println!(
                "  {}. {}: {} ({})",
                i + 1,
                tx.date.format("%Y-%m-%d"),
                tx.format_amount(),
                tx.description
            );
        }
    }

    // Calculate totals
    let total_in = recent_transactions
        .iter()
        .filter(|tx| tx.r#type == "in")
        .map(|tx| tx.to_usd())
        .sum::<f64>();

    let total_out = recent_transactions
        .iter()
        .filter(|tx| tx.r#type == "out")
        .map(|tx| tx.to_usd())
        .sum::<f64>();

    let net = total_in - total_out;

    println!("\n💰 Financial Summary (last 7 days):");
    println!("  Total Cash In: ${:.2} USD", total_in);
    println!("  Total Cash Out: ${:.2} USD", total_out);
    println!("  Net: ${:.2} USD", net);

    if net > 0.0 {
        println!("  📈 Positive cash flow: +${:.2}", net);
    } else if net < 0.0 {
        println!("  📉 Negative cash flow: ${:.2}", net);
    } else {
        println!("  ⚖️  Balanced: $0.00");
    }

    println!("\n✅ Daily tracking completed!");
    Ok(())
}
