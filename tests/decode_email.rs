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

fn main() {
    let email_text = fs::read_to_string("real_email_2.eml").unwrap();
    
    // Extract the HTML part
    let lines: Vec<&str> = email_text.lines().collect();
    let mut in_html = false;
    let mut html_content = String::new();
    
    for line in lines {
        if line.contains("Content-Type: text/html") {
            in_html = true;
            continue;
        }
        
        if in_html && line.trim().is_empty() {
            // Skip empty line after header
            continue;
        }
        
        if in_html {
            if line.starts_with("--") && line.contains("boundary") {
                break;
            }
            html_content.push_str(line);
            html_content.push('\n');
        }
    }
    
    println!("HTML Content (encoded):");
    println!("{}", &html_content[..200.min(html_content.len())]);
    println!("\n...\n");
    
    // Decode quoted-printable
    let decoded = decode_quoted_printable(&html_content);
    
    println!("Decoded HTML:");
    println!("{}", &decoded[..500.min(decoded.len())]);
    println!("\n...\n");
    
    // Extract text from HTML
    let text = decoded
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
    
    // Remove HTML tags
    let re = regex::Regex::new(r"<[^>]+>").unwrap();
    let clean_text = re.replace_all(&text, "").to_string();
    
    println!("Clean text:");
    println!("{}", clean_text);
    
    // Look for amounts
    println!("\nLooking for amounts in text:");
    
    // Check for VND
    if clean_text.contains("VND") {
        println!("Found 'VND' in text");
        let vnd_regex = regex::Regex::new(r"(\d{1,3}(?:,\d{3})*)\s*VND").unwrap();
        for cap in vnd_regex.captures_iter(&clean_text) {
            println!("VND amount: {}", &cap[1]);
        }
    }
    
    // Check for USD/$ 
    if clean_text.contains("$") || clean_text.contains("USD") {
        println!("Found '$' or 'USD' in text");
    }
    
    // Check for any numbers that look like amounts
    let amount_regex = regex::Regex::new(r"\b(\d{1,3}(?:,\d{3})*(?:\.\d{2})?)\b").unwrap();
    for cap in amount_regex.captures_iter(&clean_text) {
        println!("Possible amount: {}", &cap[1]);
    }
}