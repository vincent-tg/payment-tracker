pub mod config;
pub mod currency;
pub mod db;
pub mod email;
pub mod models;
pub mod web;
pub mod transactions;

use anyhow::Result;
use config::Config;
use db::Database;
use email::EmailClient;
use models::Transaction;
use tabled::{Table, Tabled};

pub struct App {
    config: Config,
    db: Database,
}

impl App {
    pub async fn new(config: Config) -> Result<Self> {
        let db = Database::new(&config.database.get_connection_string()).await?;
        Ok(Self { config, db })
    }
    
    pub async fn fetch_and_process_emails(&self) -> Result<()> {
        println!("Connecting to email server...");
        
        let email_client = EmailClient::new(
            &self.config.email.imap_server,
            self.config.email.imap_port,
            &self.config.email.address,
            &self.config.email.password,
        )?;
        
        println!("Fetching emails...");
        let emails = email_client.fetch_recent_emails().await?;
        
        println!("Found {} new emails", emails.len());
        
        let mut parsed_count = 0;
        let mut saved_count = 0;
        
        for (i, email) in emails.iter().enumerate() {
            if let Some(transaction) = email::parse_transaction_from_email(email) {
                parsed_count += 1;
                println!("[Email {}] Parsed transaction: {} - ${:.2} ({})", 
                    i + 1, transaction.description, transaction.amount, transaction.r#type);
                
                // Check if transaction already exists
                let exists = self.db.transaction_exists(&transaction).await?;
                
                if !exists {
                    self.db.insert_transaction(&transaction).await?;
                    saved_count += 1;
                    println!("  -> Saved to database");
                } else {
                    println!("  -> Already exists in database, skipping");
                }
            } else {
                // Debug: Print first 200 chars of emails that weren't parsed
                if i < 5 {  // Only show first 5 non-parsed emails for debugging
                    let preview = if email.len() > 200 {
                        &email[..200]
                    } else {
                        email
                    };
                    println!("[Email {}] Could not parse transaction from email (preview):\n{}...", 
                        i + 1, preview.replace("\n", " "));
                }
            }
        }
        
        println!("\nEmail processing completed!");
        println!("  Total emails: {}", emails.len());
        println!("  Parsed transactions: {}", parsed_count);
        println!("  Saved transactions: {}", saved_count);
        Ok(())
    }
    
    pub async fn list_transactions(
        &self,
        r#type: Option<String>,
        from: Option<String>,
        to: Option<String>,
        limit: Option<i64>,
    ) -> Result<()> {
        let transactions = self.db.get_transactions(r#type.as_deref(), from.as_deref(), to.as_deref(), limit).await?;
        
        if transactions.is_empty() {
            println!("No transactions found");
            return Ok(());
        }
        
        #[derive(Tabled)]
        struct TransactionRow {
            id: i64,
            date: String,
            description: String,
            amount: String,
            r#type: String,
            source: String,
        }
        
        let rows: Vec<TransactionRow> = transactions
            .iter()
            .map(|t| TransactionRow {
                id: t.id,
                date: t.date.format("%Y-%m-%d").to_string(),
                description: t.description.clone(),
                amount: format!("{:.2}", t.amount),
                r#type: t.r#type.clone(),
                source: t.source.clone(),
            })
            .collect();
        
        let table = Table::new(rows).to_string();
        println!("{}", table);
        
        let total_in: f64 = transactions.iter()
            .filter(|t| t.r#type == "in")
            .map(|t| t.amount)
            .sum();
        
        let total_out: f64 = transactions.iter()
            .filter(|t| t.r#type == "out")
            .map(|t| t.amount)
            .sum();
        
        let balance = total_in - total_out;
        
        println!("\nSummary:");
        println!("  Total In:  ${:.2}", total_in);
        println!("  Total Out: ${:.2}", total_out);
        println!("  Balance:   ${:.2}", balance);
        
        Ok(())
    }
    
    pub async fn generate_summary(&self, period: &str, date: Option<&str>) -> Result<()> {
        let summary = self.db.get_summary(period, date).await?;
        
        println!("Summary for {} period:", period);
        println!("==============================");
        println!("Total Transactions: {}", summary.total_transactions);
        println!("Total Cash In:      ${:.2}", summary.total_in);
        println!("Total Cash Out:     ${:.2}", summary.total_out);
        println!("Net Balance:        ${:.2}", summary.net_balance);
        println!("==============================");
        
        if !summary.top_categories.is_empty() {
            println!("\nTop Categories:");
            for (category, amount) in &summary.top_categories {
                println!("  {}: ${:.2}", category, amount);
            }
        }
        
        Ok(())
    }
    
    pub async fn add_manual_transaction(
        &self,
        amount: f64,
        description: &str,
        r#type: &str,
        date: Option<&str>,
    ) -> Result<()> {
        let transaction = Transaction::new_manual(amount, description, r#type, date);
        self.db.insert_transaction(&transaction).await?;
        println!("Transaction added successfully!");
        println!("  ID: {}", transaction.id);
        println!("  Date: {}", transaction.date.format("%Y-%m-%d"));
        println!("  Description: {}", transaction.description);
        println!("  Amount: ${:.2}", transaction.amount);
        println!("  Type: {}", transaction.r#type);
        Ok(())
    }
}