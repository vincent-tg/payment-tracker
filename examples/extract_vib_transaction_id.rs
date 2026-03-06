use std::fs;

fn decode_quoted_printable(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '=' {
            if let Some(&next) = chars.peek() {
                if next == '\n' {
                    chars.next();
                    continue;
                }
                if next.is_ascii_hexdigit() {
                    let hex: String = chars.by_ref().take(2).collect();
                    if hex.len() == 2 {
                        if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                            result.push(byte as char);
                        }
                        continue;
                    }
                }
            }
            result.push('=');
        } else {
            result.push(c);
        }
    }
    
    result
}

fn main() -> anyhow::Result<()> {
    println!("Extracting Transaction IDs from VIB Bank Email");
    println!("==============================================\n");
    
    let email_text = fs::read_to_string("real_email_2.eml")?;
    
    // Look for transaction ID patterns
    println!("Searching for transaction IDs in email...");
    
    // Common patterns for transaction IDs
    let patterns = vec![
        r"Mã giao dịch",
        r"Mã GD",
        r"Transaction ID",
        r"Ref\.",
        r"Reference",
        r"ID:",
        r"Số giao dịch",
        r"GD\s*[0-9]+",
        r"#[0-9]+",
    ];
    
    // Decode quoted-printable sections
    let lines: Vec<&str> = email_text.lines().collect();
    let mut in_html = false;
    let mut html_content = String::new();
    
    for line in lines {
        if line.contains("Content-Type: text/html") {
            in_html = true;
            continue;
        }
        
        if in_html && line.trim().is_empty() {
            continue;
        }
        
        if in_html {
            if line.starts_with("--") && line.contains("boundary") {
                break;
            }
            html_content.push_str(line);
        }
    }
    
    // Decode the HTML
    let decoded_html = decode_quoted_printable(&html_content);
    
    println!("Decoded HTML (first 1000 chars):");
    println!("{}", &decoded_html[..1000.min(decoded_html.len())]);
    println!("\n...\n");
    
    // Search for patterns
    println!("Looking for transaction ID patterns:");
    for pattern in patterns {
        if decoded_html.contains(pattern) {
            println!("✅ Found pattern: '{}'", pattern);
            // Extract context
            if let Some(pos) = decoded_html.find(pattern) {
                let start = pos.saturating_sub(50);
                let end = (pos + 100).min(decoded_html.len());
                println!("   Context: ...{}...", &decoded_html[start..end]);
            }
        }
    }
    
    // Also look for any alphanumeric IDs that might be transaction IDs
    println!("\nLooking for potential transaction IDs (alphanumeric, 6+ chars):");
    let re = regex::Regex::new(r"\b[A-Z0-9]{6,}\b").unwrap();
    for cap in re.captures_iter(&decoded_html) {
        let id = &cap[0];
        // Skip common non-ID strings
        if !id.contains("HTTP") && !id.contains("HTML") && !id.contains("DOCTYPE") {
            println!("   Potential ID: {}", id);
        }
    }
    
    // Check for Vietnamese transaction references
    println!("\nChecking for Vietnamese transaction references:");
    let vietnamese_patterns = vec![
        "Mã giao dịch",
        "Số giao dịch", 
        "Mã GD",
        "Mã tham chiếu",
        "Tham chiếu",
    ];
    
    for pattern in vietnamese_patterns {
        if decoded_html.contains(pattern) {
            println!("✅ Found: '{}'", pattern);
        }
    }
    
    Ok(())
}