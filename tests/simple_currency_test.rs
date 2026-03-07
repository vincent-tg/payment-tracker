use payment_tracker::email;
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("Simple Currency Detection Test");
    println!("==============================\n");
    
    // Read the real email
    let email_text = fs::read_to_string("real_email_2.eml")?;
    
    println!("Parsing real_email_2.eml...\n");
    
    match email::parse_transaction_from_email(&email_text) {
        Some(transaction) => {
            println!("✅ Transaction parsed successfully!");
            println!("\nParsed Details:");
            println!("  Amount: {}", transaction.format_amount());
            println!("  Currency: {}", transaction.currency);
            println!("  Bank: {}", transaction.bank);
            println!("  Type: {}", transaction.r#type);
            println!("  Description: {}", transaction.description);
            println!("  Date: {}", transaction.date);
            
            // Show conversion
            if transaction.currency != "USD" {
                println!("  USD Equivalent: ${:.2}", transaction.to_usd());
            }
            
            // Check if this looks like a VIB transaction
            println!("\nAnalysis:");
            if transaction.bank == "VIB" {
                println!("  ✓ Identified as VIB bank transaction");
            }
            if transaction.currency == "VND" {
                println!("  ✓ Currency is VND (Vietnamese Dong)");
                println!("  ✓ Amount: {:.0} VND", transaction.amount);
                
                // Check if amount is reasonable for VND
                if transaction.amount > 10000.0 && transaction.amount < 1000000.0 {
                    println!("  ✓ Amount is reasonable for VND transaction");
                }
            } else {
                println!("  ⚠️  Currency is {} (expected VND)", transaction.currency);
            }
            
            // Check the amount value
            if transaction.amount == 58000.0 {
                println!("  ✓ Correct amount: 58,000 VND");
            } else if transaction.amount == 12345000.0 {
                println!("  ⚠️  Amount is 12,345,000 VND (this might be account balance, not transaction)");
            }
        }
        None => {
            println!("❌ Failed to parse transaction");
        }
    }
    
    // Let me also check the raw email for VND mentions
    println!("\n\nChecking raw email for 'VND':");
    let vnd_count = email_text.matches("VND").count();
    println!("  Found 'VND' {} times in email", vnd_count);
    
    if vnd_count > 0 {
        // Find first occurrence
        if let Some(pos) = email_text.find("VND") {
            let start = pos.saturating_sub(50);
            let end = (pos + 10).min(email_text.len());
            println!("  Context: ...{}...", &email_text[start..end]);
        }
    }
    
    Ok(())
}