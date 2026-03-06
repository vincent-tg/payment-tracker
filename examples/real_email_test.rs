use payment_tracker::email;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Real Email Fetching and Parsing");
    println!("========================================\n");
    
    // Read config from known path
    let config_path = "/home/ubuntu/.payment-tracker/config.toml";
    
    let config_content = fs::read_to_string(&config_path)?;
    let config: toml::Value = toml::from_str(&config_content)?;
    
    let email_config = config.get("email").unwrap();
    let server = email_config.get("imap_server").unwrap().as_str().unwrap();
    let port = email_config.get("imap_port").unwrap().as_integer().unwrap() as u16;
    let username = email_config.get("address").unwrap().as_str().unwrap();
    let password = email_config.get("password").unwrap().as_str().unwrap();
    
    println!("Connecting to: {}:{}", server, port);
    println!("Username: {}", username);
    println!("Password: [PROTECTED]\n");
    
    // Create email client
    let client = email::EmailClient::new(server, port, username, password)?;
    
    println!("Attempting to fetch recent emails...");
    
    // Try to fetch emails
    match tokio::runtime::Runtime::new()?.block_on(client.fetch_recent_emails()) {
        Ok(emails) => {
            println!("Successfully fetched {} emails", emails.len());
            
            if emails.is_empty() {
                println!("No emails found in the last 7 days.");
                return Ok(());
            }
            
            println!("\nAnalyzing email content...");
            
            // Save first few emails for analysis
            for (i, email_text) in emails.iter().take(3).enumerate() {
                let filename = format!("real_email_{}.eml", i + 1);
                fs::write(&filename, email_text)?;
                println!("Saved email {} to: {}", i + 1, filename);
                
                // Try to parse it
                match email::parse_transaction_from_email(email_text) {
                    Some(transaction) => {
                        println!("  ✅ Parsed transaction!");
                        println!("     Amount: ${}", transaction.amount);
                        println!("     Type: {}", transaction.r#type);
                        println!("     Description: {}", transaction.description);
                        println!("     Date: {}", transaction.date);
                    }
                    None => {
                        println!("  ❌ Not a bank transaction email (or parsing failed)");
                        
                        // Let's check what's in the email
                        let lines: Vec<&str> = email_text.lines().collect();
                        let subject = lines.iter().find(|l| l.starts_with("Subject:"));
                        if let Some(subj) = subject {
                            println!("  Subject: {}", subj);
                        }
                        
                        // Check for common bank keywords
                        let body = email_text.to_lowercase();
                        let keywords = ["bank", "transaction", "payment", "amount", "$", "purchase", "charge", "debit", "credit"];
                        let found: Vec<&str> = keywords.iter()
                            .filter(|kw| body.contains(**kw))
                            .map(|kw| *kw)
                            .collect();
                        
                        if !found.is_empty() {
                            println!("  Contains keywords: {:?}", found);
                            println!("  This might be a bank email that needs better parsing!");
                        }
                    }
                }
                println!();
            }
            
            // Try to parse all emails for transactions
            println!("Parsing all emails for transactions...");
            match tokio::runtime::Runtime::new()?.block_on(client.fetch_and_parse_transactions()) {
                Ok(transactions) => {
                    println!("Found {} transactions in emails", transactions.len());
                    for (i, t) in transactions.iter().enumerate() {
                        println!("  {}. ${} - {} - {} - {}", 
                            i + 1, t.amount, t.r#type, t.description, t.date);
                    }
                }
                Err(e) => {
                    println!("Error parsing transactions: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to fetch emails: {}", e);
            println!("Possible issues:");
            println!("1. Email credentials incorrect");
            println!("2. IMAP access not enabled in Gmail");
            println!("3. App password needed (not regular password)");
            println!("4. Network/firewall issue");
        }
    }
    
    Ok(())
}