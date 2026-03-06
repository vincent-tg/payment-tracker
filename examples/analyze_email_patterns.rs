use payment_tracker::email;
use regex::Regex;

fn analyze_email_pattern(email_text: &str, name: &str) {
    println!("\n=== Analyzing {} Email ===", name);
    
    // Try to parse the transaction
    match email::parse_transaction_from_email(email_text) {
        Some(transaction) => {
            println!("✅ Successfully parsed!");
            println!("   Amount: ${}", transaction.amount);
            println!("   Type: {}", transaction.r#type);
            println!("   Description: {}", transaction.description);
            println!("   Date: {}", transaction.date);
        }
        None => {
            println!("❌ Failed to parse");
            
            // Let's analyze why it might have failed
            let parsed = mailparse::parse_mail(email_text.as_bytes()).unwrap();
            let body = extract_body_for_analysis(&parsed);
            
            println!("   Email body preview (first 500 chars):");
            println!("   {}", &body.chars().take(500).collect::<String>());
            
            // Check for amount patterns
            let amount_patterns = vec![
                r"\$[\d,]+\.\d{2}",
                r"USD\s+[\d,]+\.\d{2}",
                r"[\d,]+\.\d{2}\s*(?:USD|EUR|GBP)",
                r"Amount:\s*[\$€£]?\s*[\d,]+\.\d{2}",
                r"Total:\s*[\$€£]?\s*[\d,]+\.\d{2}",
            ];
            
            println!("\n   Looking for amount patterns:");
            for pattern in amount_patterns {
                if let Ok(re) = Regex::new(pattern) {
                    if let Some(cap) = re.find(&body) {
                        println!("   Found: {}", cap.as_str());
                    }
                }
            }
        }
    }
}

fn extract_body_for_analysis(parsed_mail: &mailparse::ParsedMail) -> String {
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
                    
                    let re = Regex::new(r"<[^>]+>").unwrap();
                    *html_body = re.replace_all(&cleaned, "").to_string();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    println!("Analyzing Email Patterns for Payment Tracker");
    println!("============================================\n");
    
    // Analyze all sample emails
    let samples = vec![
        ("Generic Bank", email::get_example_bank_email()),
        ("Chase", email::get_chase_email_sample()),
        ("PayPal", email::get_paypal_email_sample()),
        ("Venmo", email::get_venmo_email_sample()),
        ("Bank of America (Multipart)", email::get_multipart_html_email_sample()),
        ("Wells Fargo (Base64)", email::get_base64_encoded_email_sample()),
        ("Zelle", email::get_zelle_email_sample()),
        ("Cash App", email::get_cashapp_email_sample()),
        ("Capital One", email::get_credit_card_email_sample()),
    ];
    
    for (name, email_text) in samples {
        analyze_email_pattern(&email_text, name);
    }
    
    println!("\n=== Summary of Email Patterns ===");
    println!("1. Amount formats found:");
    println!("   - $XX.XX (most common)");
    println!("   - USD XX.XX");
    println!("   - XX.XX (no currency symbol)");
    println!("   - €XX.XX, £XX.XX");
    
    println!("\n2. Date formats:");
    println!("   - MM/DD/YYYY (01/15/2024)");
    println!("   - Month DD, YYYY (January 15, 2024)");
    println!("   - DD/MM/YYYY (15/01/2024)");
    println!("   - YYYY-MM-DD (2024-01-15)");
    
    println!("\n3. Description/Merchant labels:");
    println!("   - Merchant:");
    println!("   - Description:");
    println!("   - Sent to:");
    println!("   - Paid to:");
    println!("   - For:");
    
    println!("\n4. Transaction type indicators:");
    println!("   - Debit/Debited/Purchase/Paid/Sent → 'out'");
    println!("   - Credit/Credited/Received/Paid you → 'in'");
    
    println!("\n✅ Analysis complete. Use these patterns to improve parsing!");
}