use payment_tracker::email;
use std::fs;

fn main() {
    println!("Testing VND Currency Parsing");
    println!("============================\n");
    
    // Test 1: Parse the real Vietnamese bank email
    println!("1. Testing real Vietnamese bank email:");
    let email_text = fs::read_to_string("real_email_2.eml").unwrap();
    
    match email::parse_transaction_from_email(&email_text) {
        Some(transaction) => {
            println!("   ✅ Successfully parsed!");
            println!("     Amount: {} VND", transaction.amount);
            println!("     Type: {}", transaction.r#type);
            println!("     Description: {}", transaction.description);
            println!("     Date: {}", transaction.date);
            
            // Convert VND to USD for reference (approx 23,000 VND = 1 USD)
            let usd_amount = transaction.amount / 23000.0;
            println!("     ≈ ${:.2} USD", usd_amount);
        }
        None => {
            println!("   ❌ Still failed to parse");
            
            // Debug: extract body and check for patterns
            if let Ok(parsed) = mailparse::parse_mail(email_text.as_bytes()) {
                let mut best_body = String::new();
                let mut html_body = String::new();
                
                extract_body_recursive(&parsed, &mut best_body, &mut html_body);
                let body = if !best_body.is_empty() { best_body } else { html_body };
                
                println!("   Body preview (first 300 chars):");
                println!("   {}", &body.chars().take(300).collect::<String>());
            }
        }
    }
    
    println!("\n2. Testing VND pattern matching:");
    
    // Test various VND formats
    let test_cases = vec![
        "58,000 VND",
        "1000000 VND",
        "1,000,000 VND",
        "Giá trị: 58,000 VND",
        "Amount: 58,000 VND",
        "Total: 1,000,000 VND",
    ];
    
    for test in test_cases {
        let test_email = format!("Content-Type: text/plain\n\n{}", test);
        match email::parse_transaction_from_email(&test_email) {
            Some(t) => println!("   ✅ '{}' → {} VND", test, t.amount),
            None => println!("   ❌ '{}' → failed", test),
        }
    }
    
    println!("\n3. Testing Vietnamese date parsing:");
    
    let date_tests = vec![
        "08:51 03/03/2026",
        "Date: 03/03/2026",
        "Vào lúc: 08:51 03/03/2026",
        "2026-03-03",
    ];
    
    for test in date_tests {
        let test_email = format!("Content-Type: text/plain\n\n{}", test);
        if let Some(t) = email::parse_transaction_from_email(&test_email) {
            println!("   ✅ '{}' → {}", test, t.date);
        }
    }
}

fn extract_body_recursive(parsed_mail: &mailparse::ParsedMail, best_body: &mut String, html_body: &mut String) {
    let mimetype = &parsed_mail.ctype.mimetype;
    
    if mimetype.starts_with("multipart/") {
        for part in &parsed_mail.subparts {
            extract_body_recursive(part, best_body, html_body);
            if !best_body.is_empty() {
                return;
            }
        }
        return;
    }
    
    if let Ok(content) = parsed_mail.get_body() {
        let text = String::from_utf8_lossy(content.as_bytes()).to_string();
        if text.trim().is_empty() {
            return;
        }
        
        match mimetype.as_str() {
            "text/plain" => {
                if best_body.is_empty() {
                    *best_body = text;
                }
            }
            "text/html" => {
                if html_body.is_empty() {
                    // Simple HTML to text conversion
                    let cleaned = text
                        .replace("<br>", "\n")
                        .replace("<br/>", "\n")
                        .replace("<br />", "\n")
                        .replace("</p>", "\n")
                        .replace("</div>", "\n")
                        .replace("</tr>", "\n")
                        .replace("</td>", " ")
                        .replace("&nbsp;", " ")
                        .replace("&amp;", "&")
                        .replace("&lt;", "<")
                        .replace("&gt;", ">")
                        .replace("&quot;", "\"");
                    
                    let re = regex::Regex::new(r"<[^>]+>").unwrap();
                    *html_body = re.replace_all(&cleaned, "").to_string();
                }
            }
            _ => {}
        }
    }
}