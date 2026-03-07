use payment_tracker::email;
use std::fs;

fn main() -> anyhow::Result<()> {
    println!("Debugging Currency Detection in Real Emails");
    println!("===========================================\n");

    // Read the real email file
    let email_text = fs::read_to_string("real_email_2.eml")?;

    println!("Analyzing real_email_2.eml...");
    println!("Email size: {} bytes", email_text.len());

    // Parse the email to get body
    let parsed = mailparse::parse_mail(email_text.as_bytes()).unwrap();
    let body = email::extract_email_body(&parsed);

    println!("\nLooking for currency indicators in email body:");
    println!("------------------------------------------------");

    // Check for VND
    if body.contains("VND") {
        println!("✅ Found 'VND' in email body");
        // Find context around VND
        let vnd_index = body.find("VND").unwrap();
        let start = vnd_index.saturating_sub(100);
        let end = (vnd_index + 50).min(body.len());
        println!("Context around 'VND':");
        println!("...{}...", &body[start..end]);
    } else {
        println!("❌ 'VND' NOT found in email body");
    }

    // Check for USD/$
    if body.contains("USD") {
        println!("✅ Found 'USD' in email body");
    }
    if body.contains("$") {
        println!("✅ Found '$' in email body");
    }

    // Check for Vietnamese currency keywords
    let vietnamese_keywords = ["giá trị", "Giá trị", "GIÁ TRỊ", "VND", "vnd"];
    for keyword in vietnamese_keywords {
        if body.contains(keyword) {
            println!("✅ Found Vietnamese keyword: '{}'", keyword);
        }
    }

    // Now parse the transaction
    println!("\n\nParsing transaction from real email:");
    println!("-------------------------------------");

    match email::parse_transaction_from_email(&email_text) {
        Some(transaction) => {
            println!("✅ Transaction parsed!");
            println!("\nParsed Transaction:");
            println!("  Amount: {}", transaction.format_amount());
            println!("  Currency: {}", transaction.currency);
            println!("  Bank: {}", transaction.bank);
            println!("  Type: {}", transaction.r#type);
            println!("  Description: {}", transaction.description);
            println!("  Date: {}", transaction.date);

            // Check if currency is correct
            if transaction.currency == "VND" {
                println!("\n✅ CORRECT: Currency is VND");
            } else {
                println!(
                    "\n❌ PROBLEM: Currency should be VND but is {}",
                    transaction.currency
                );
            }

            // Check amount
            if transaction.amount == 58000.0 {
                println!("✅ CORRECT: Amount is 58,000 VND");
            } else {
                println!("❌ Amount is {} (expected 58000)", transaction.amount);
            }

            // Check bank
            if transaction.bank == "VIB" {
                println!("✅ CORRECT: Bank is VIB");
            } else {
                println!("❌ Bank is {} (should be VIB)", transaction.bank);
            }
        }
        None => {
            println!("❌ Failed to parse transaction");
        }
    }

    // Let's also check what the email body looks like after processing
    println!("\n\nEmail body preview (first 500 chars):");
    println!("--------------------------------------");
    println!("{}", &body[..500.min(body.len())]);

    Ok(())
}
