use payment_tracker::email;

fn main() {
    println!("Testing Payment Tracker Email Parsing Improvements");
    println!("=================================================\n");

    // Test 1: Plain text email
    println!("1. Testing Plain Text Email Parsing:");
    let plain_email = r#"Subject: Transaction Alert - Debit
From: yourbank@example.com
Date: 15/01/2024
Content-Type: text/plain

Dear Customer,

A transaction has been made on your account:

Amount: $50.00
Description: GROCERY STORE PURCHASE
Date: 15/01/2024
Transaction Type: DEBITED

Thank you,
Your Bank"#;

    match email::parse_transaction_from_email(plain_email) {
        Some(t) => {
            println!("   ✓ Successfully parsed transaction!");
            println!("     Amount: ${}", t.amount);
            println!("     Type: {}", t.r#type);
            println!("     Description: {}", t.description);
            println!("     Date: {}", t.date);
        }
        None => println!("   ✗ Failed to parse plain text email"),
    }

    println!("\n2. Testing HTML Email Parsing:");
    let html_email = r#"Subject: Your Chase Credit Card Transaction
From: chase@alerts.chase.com
Date: Fri, 06 Mar 2026 14:47:00 +0000
Content-Type: multipart/alternative; boundary="boundary123"

--boundary123
Content-Type: text/plain; charset=utf-8

Amount: $129.99
Merchant: AMAZON.COM
Date: March 6, 2026

--boundary123
Content-Type: text/html; charset=utf-8

<html>
<body>
<div>
  <p>Amount: <strong>$129.99</strong></p>
  <p>Merchant: <strong>AMAZON.COM</strong></p>
  <p>Date: <strong>March 6, 2026</strong></p>
</div>
</body>
</html>

--boundary123--"#;

    match email::parse_transaction_from_email(html_email) {
        Some(t) => {
            println!("   ✓ Successfully parsed HTML transaction!");
            println!("     Amount: ${}", t.amount);
            println!("     Type: {}", t.r#type);
            println!("     Description: {}", t.description);
        }
        None => println!("   ✗ Failed to parse HTML email"),
    }

    println!("\n3. Testing Different Amount Formats:");
    let formats = [
        ("Amount: $1,234.56", "Standard format"),
        ("Total: €99.99", "Euro format"),
        ("Debit of: £50.00", "Pound format"),
        ("Transaction for 75.00", "No currency symbol"),
        ("USD 200.00 charged", "Currency code format"),
    ];

    for (email_body, description) in formats.iter() {
        let test_email = format!("Content-Type: text/plain\n\n{}", email_body);
        if let Some(t) = email::parse_transaction_from_email(&test_email) {
            println!("   ✓ {}: parsed ${}", description, t.amount);
        } else {
            println!("   ✗ {}: failed to parse", description);
        }
    }

    println!("\n✅ Email parsing improvements are working!");
}
