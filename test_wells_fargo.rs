use payment_tracker::email;

fn main() {
    println!("Testing Wells Fargo Base64 Email Parsing");
    println!("========================================\n");
    
    let wells_fargo_email = email::get_base64_encoded_email_sample();
    println!("Email sample:");
    println!("{}", wells_fargo_email);
    println!();
    
    // Try to parse it
    match email::parse_transaction_from_email(&wells_fargo_email) {
        Some(t) => {
            println!("✅ Successfully parsed!");
            println!("   Amount: ${}", t.amount);
            println!("   Type: {}", t.r#type);
            println!("   Description: {}", t.description);
            println!("   Date: {}", t.date);
        }
        None => {
            println!("❌ Failed to parse");
            
            // Let's manually decode the base64 to see what it says
            let lines: Vec<&str> = wells_fargo_email.lines().collect();
            let mut in_base64_section = false;
            let mut base64_content = String::new();
            
            for line in lines {
                if line.trim() == "Content-Transfer-Encoding: base64" {
                    in_base64_section = true;
                    continue;
                }
                
                if in_base64_section && line.trim().is_empty() {
                    // Skip empty line after header
                    continue;
                }
                
                if in_base64_section {
                    if line.contains(".=") {
                        // End of base64 section
                        let base64_part = line.split(".=").next().unwrap_or("");
                        base64_content.push_str(base64_part);
                        break;
                    } else {
                        base64_content.push_str(line.trim());
                    }
                }
            }
            
            println!("\nBase64 content found: {}", base64_content);
            
            // Try to decode it
            use base64::Engine;
            match base64::engine::general_purpose::STANDARD.decode(&base64_content) {
                Ok(decoded) => {
                    let text = String::from_utf8_lossy(&decoded);
                    println!("Decoded text: {}", text);
                    
                    // Check if it contains amount
                    if text.contains("$59.99") {
                        println!("✅ Contains expected amount: $59.99");
                    }
                }
                Err(e) => {
                    println!("Failed to decode base64: {}", e);
                }
            }
        }
    }
}