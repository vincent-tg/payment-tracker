use payment_tracker::email;
use std::fs;

fn main() {
    println!("Testing Vietnamese Bank Email Parsing");
    println!("=====================================\n");
    
    // Read the real Vietnamese bank email
    let email_text = fs::read_to_string("real_email_2.eml").unwrap();
    
    println!("Email contains Vietnamese bank transaction:");
    println!("- Amount: 58,000 VND");
    println!("- Merchant: 7ELEVEN_1062");
    println!("- Date: 03/03/2026 08:51");
    println!();
    
    // Try to parse it with current parser
    match email::parse_transaction_from_email(&email_text) {
        Some(transaction) => {
            println!("✅ Current parser successfully parsed!");
            println!("   Amount: ${}", transaction.amount);
            println!("   Type: {}", transaction.r#type);
            println!("   Description: {}", transaction.description);
            println!("   Date: {}", transaction.date);
        }
        None => {
            println!("❌ Current parser failed to parse Vietnamese bank email");
            println!();
            
            // Let's analyze why
            println!("Analysis of why parsing failed:");
            println!("1. Currency: Email uses '58,000 VND', parser looks for '$', 'USD', '€', '£'");
            println!("2. Language: Email is in Vietnamese, parser keywords are in English");
            println!("3. Format: Specific Vietnamese bank format not recognized");
            println!();
            
            // Extract the decoded HTML body to see actual text
            if let Ok(parsed) = mailparse::parse_mail(email_text.as_bytes()) {
                let body = extract_body_text(&parsed);
                println!("Decoded email body (first 500 chars):");
                println!("{}", &body.chars().take(500).collect::<String>());
                println!();
                
                // Check for VND pattern
                let vnd_pattern = r"(\d{1,3}(?:,\d{3})*)\s*VND";
                let re = regex::Regex::new(vnd_pattern).unwrap();
                if let Some(cap) = re.find(&body) {
                    println!("Found VND amount: {}", cap.as_str());
                    let amount_str = cap.as_str().replace(" VND", "").replace(",", "");
                    if let Ok(amount) = amount_str.parse::<f64>() {
                        println!("Parsed amount: {} VND (≈ ${:.2} USD)", amount, amount / 23000.0);
                    }
                }
                
                // Check for date
                let date_pattern = r"(\d{2}:\d{2}\s+\d{2}/\d{2}/\d{4})";
                let re_date = regex::Regex::new(date_pattern).unwrap();
                if let Some(cap) = re_date.find(&body) {
                    println!("Found date/time: {}", cap.as_str());
                }
                
                // Check for merchant
                if body.contains("7ELEVEN") {
                    println!("Found merchant: 7ELEVEN");
                }
            }
        }
    }
    
    println!("\n=== Recommendations ===");
    println!("1. Add VND currency support to amount patterns");
    println!("2. Add Vietnamese transaction keywords (e.g., 'Giao dịch', 'Thanh toán')");
    println!("3. Handle Vietnamese date formats (dd/mm/yyyy)");
    println!("4. Consider bank-specific parsers for common Vietnamese banks");
}

fn extract_body_text(parsed_mail: &mailparse::ParsedMail) -> String {
    let mut best_body = String::new();
    let mut html_body = String::new();
    
    extract_body_recursive(parsed_mail, &mut best_body, &mut html_body);
    
    if !best_body.is_empty() {
        best_body
    } else if !html_body.is_empty() {
        html_body
    } else {
        String::new()
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