use payment_tracker::email;
use std::fs;

fn main() {
    println!("Testing Specialized Vietnamese Bank Email Parser");
    println!("================================================\n");
    
    // Test 1: Real Vietnamese bank email
    println!("1. Testing real Vietnamese bank email (real_email_2.eml):");
    let email_text = fs::read_to_string("real_email_2.eml").unwrap();
    
    match email::parse_transaction_from_email(&email_text) {
        Some(transaction) => {
            println!("   ✅ Successfully parsed!");
            println!("     Amount: {} VND", transaction.amount);
            println!("     Type: {}", transaction.r#type);
            println!("     Description: {}", transaction.description);
            println!("     Date: {}", transaction.date);
            
            // Convert VND to USD for reference
            let usd_amount = transaction.amount / 23000.0;
            println!("     ≈ ${:.2} USD", usd_amount);
            
            // Check if it's correct
            if transaction.amount == 58000.0 {
                println!("     ✓ Correct amount: 58,000 VND");
            } else {
                println!("     ✗ Wrong amount: expected 58,000 VND, got {}", transaction.amount);
            }
            
            if transaction.r#type == "out" {
                println!("     ✓ Correct type: out (debit)");
            } else {
                println!("     ✗ Wrong type: expected 'out', got '{}'", transaction.r#type);
            }
            
            if transaction.description.contains("7ELEVEN") {
                println!("     ✓ Correct merchant: 7ELEVEN");
            }
        }
        None => {
            println!("   ❌ Failed to parse");
        }
    }
    
    println!("\n2. Testing other email patterns:");
    
    // Test various bank email formats
    let test_cases = vec![
        (
            "Vietnamese VIB Bank",
            r#"Content-Type: text/html
Content-Transfer-Encoding: quoted-printable

<div>Gi=C3=A1 tr=E1=BB=8B: <b>58,000 VND</b></div>
<div>V=C3=A0o l=C3=BAc: <b>08:51 03/03/2026</b></div>
<div>T=E1=BA=A1i <b>7ELEVEN_1062</b></div>"#
        ),
        (
            "US Bank with $",
            r#"Content-Type: text/plain

Transaction Alert
Amount: $129.99
Merchant: AMAZON.COM
Date: 03/03/2026"#
        ),
        (
            "Vietnamese with 'giá trị'",
            r#"Content-Type: text/plain

Giá trị: 150,000 VND
Tại: COFFEE SHOP
Thời gian: 14:30 03/03/2026"#
        ),
    ];
    
    for (name, email_text) in test_cases {
        println!("\n   Testing {}:", name);
        match email::parse_transaction_from_email(email_text) {
            Some(t) => {
                println!("     ✅ Amount: {} - Type: {} - Desc: {}", t.amount, t.r#type, t.description);
            }
            None => {
                println!("     ❌ Failed to parse");
            }
        }
    }
    
    println!("\n=== Summary ===");
    println!("The parser is now specialized for:");
    println!("1. Vietnamese bank emails (VND currency)");
    println!("2. Vietnamese keywords: 'giá trị', 'tại', 'VND'");
    println!("3. Proper currency detection (VND before generic numbers)");
    println!("4. Transaction context awareness");
}