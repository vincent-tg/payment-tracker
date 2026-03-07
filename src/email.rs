use anyhow::{Result, anyhow};
use base64::Engine;
use chrono::{Local, NaiveDate};
use imap;
use mailparse::*;
use native_tls::TlsConnector;
use regex::Regex;
use std::env;


use crate::models::Transaction;

pub struct EmailClient {
    server: String,
    port: u16,
    username: String,
    password: String,
}

impl EmailClient {
    pub fn new(server: &str, port: u16, username: &str, password: &str) -> Result<Self> {
        // Prefer the provided password; fall back to env if empty
        let final_password = if password.trim().is_empty() {
            env::var("EMAIL_APP_PASSWORD").unwrap_or_default()
        } else {
            password.to_string()
        };

        if final_password.is_empty() {
            return Err(anyhow!("Email password not provided in config and EMAIL_APP_PASSWORD environment variable not set"));
        }

        Ok(Self {
            server: server.to_string(),
            port,
            username: username.to_string(),
            password: final_password,
        })
    }
    
    pub async fn fetch_recent_emails(&self) -> Result<Vec<String>> {
        println!("Connecting to IMAP server: {}:{}", self.server, self.port);
        
        // Create TLS connector
        let tls = TlsConnector::builder()
            .build()
            .map_err(|e| anyhow!("Failed to create TLS connector: {}", e))?;
        
        // Connect to IMAP server
        let client = imap::connect((self.server.as_str(), self.port), &self.server, &tls)
            .map_err(|e| anyhow!("Failed to connect to IMAP server: {}", e))?;
        
        println!("Logging in as {}...", self.username);
        let mut imap_session = match client.login(&self.username, &self.password) {
            Ok(session) => session,
            Err((err, _client)) => return Err(anyhow!("Failed to login: {}", err)),
        };
        
        // Select INBOX
        imap_session.select("INBOX")
            .map_err(|e| anyhow!("Failed to select INBOX: {}", e))?;
        
        println!("Fetching ALL emails from last 7 days (including read)...");
        
        // Search for ALL emails from last 7 days (not just unread)
        let since_date = chrono::Utc::now() - chrono::Duration::days(7);
        let date_str = since_date.format("%d-%b-%Y").to_string();
        let search_query = format!("SINCE {}", date_str);
        
        let message_ids = imap_session.search(search_query)
            .map_err(|e| anyhow!("Failed to search emails: {}", e))?;
        
        println!("Found {} new emails", message_ids.len());
        
        let mut email_contents = Vec::new();
        
        if !message_ids.is_empty() {
            // Fetch email bodies
            let fetch_result = imap_session.fetch(message_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(","), "RFC822")
                .map_err(|e| anyhow!("Failed to fetch emails: {}", e))?;
            
            for message in &fetch_result {
                if let Some(body) = message.body() {
                    let body_str = std::str::from_utf8(body)
                        .map_err(|e| anyhow!("Failed to parse email body as UTF-8: {}", e))?;
                    
                    email_contents.push(body_str.to_string());
                }
            }
            
            // Don't mark emails as read during testing
            // if !message_ids.is_empty() {
            //     let _ = imap_session.store(message_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(","), "+FLAGS (\\Seen)");
            // }
        }
        
        // Logout
        imap_session.logout()
            .map_err(|e| anyhow!("Failed to logout: {}", e))?;
        
        println!("Successfully fetched {} emails", email_contents.len());
        Ok(email_contents)
    }
    
    pub async fn fetch_and_parse_transactions(&self) -> Result<Vec<Transaction>> {
        let emails = self.fetch_recent_emails().await?;
        let mut transactions = Vec::new();
        
        for email in emails {
            if let Some(transaction) = parse_transaction_from_email(&email) {
                transactions.push(transaction);
            }
        }
        
        Ok(transactions)
    }
}

pub fn parse_transaction_from_email(email_text: &str) -> Option<Transaction> {
    // Parse the email using mailparse
    let parsed = match mailparse::parse_mail(email_text.as_bytes()) {
        Ok(parsed) => parsed,
        Err(_) => return None,
    };
    
    // Extract email message ID from headers for upsert tracking
    let email_message_id = parsed
        .headers
        .get_first_value("Message-ID")
        .or_else(|| parsed.headers.get_first_value("Message-Id"))
        .or_else(|| parsed.headers.get_first_value("message-id"));
    
    // Get the email body (prefer plain text, fall back to HTML)
    let body = extract_email_body(&parsed);
    
    if body.is_empty() {
        return None;
    }
    
    // Comprehensive regex patterns for bank email formats
    let patterns = vec![
        // Chase-specific patterns
        (r"(?i)Chase\s+(?:debit|credit|charge|payment)\s+(?:of|for)?\s*[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        (r"(?i)Chase\s+(?:sent|received|approved)\s+[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        (r"(?i)You (?:sent|paid|received|transferred)\s+[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        
        // Bank of America patterns
        (r"(?i)Bank of America\s*[-–]\s*.*?[$€£¥]\s*([\d,]+\.?\d{2})", 1),
        (r"(?i)BofA\s*.*?[$€£¥]\s*([\d,]+\.?\d{2})", 1),
        
        // Wells Fargo patterns
        (r"(?i)Wells Fargo\s*[-–]\s*.*?[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        (r"(?i)WellsFargo.*?(?:debited|credited)\s*[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        
        // Capital One patterns
        (r"(?i)Capital One\s*[-–]\s*.*?[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        (r"(?i)Purchase\s+(?:of|for)?\s*[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        
        // Citibank patterns
        (r"(?i)Citi\s*[-–]\s*.*?[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        
        // US Bank patterns
        (r"(?i)U\.?S\.?\s*Bank\s*[-–]\s*.*?[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        
        // PayPal patterns
        (r"(?i)PayPal\s+(?:payment|transaction|charge)\s+.*?[$€£¥]\s*([\d,]+\.?\d{2})", 1),
        (r"(?i)You (?:sent|received)\s+(?:a\s+)?payment\s+(?:of\s+)?[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        
        // Venmo patterns
        (r"(?i)Venmo\s+(?:payment|charge)\s+.*?[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        (r"(?i)paid?\s+(?:you|[\w\s]+)\s+[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        
        // Zelle patterns
        (r"(?i)Zelle\s+(?:payment|transaction)\s+.*?[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        (r"(?i)sent\s+(?:you|[\w\s]+)\s+[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        
        // Cash App patterns
        (r"(?i)Cash App\s+(?:payment|charge)\s+.*?[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        (r"(?i)Payment\s+(?:of|for)?\s+[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        
        // Generic transaction alert patterns
        (r"(?i)(?:transaction|purchase|payment|debit|credit)\s+(?:alert|notice|notification)\s*[-–]?\s*[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        
        // Pattern 1: VND XX,XXX format (Vietnamese Dong, no decimal) - SPECIFIC
        (r"(?i)(\d{1,3}(?:,\d{3})*)\s*VND\b", 1),
        // Pattern 2: USD XX.XX format - SPECIFIC
        (r"(?i)(?:USD|EUR|GBP|JPY|AUD|CAD)\s*([\d,]+(?:\.\d{2})?)\b", 1),
        // Pattern 3: Amount: $XX.XX format (most common)
        (r"(?i)(?:amount|transaction\s+amount|total\s+amount|payment\s+amount|giá\s+trị|giá trị)\s*[:=]\s*[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        // Pattern 4: $XX.XX amount format (standalone)
        (r"(?i)[$€£¥]\s*([\d,]+\.?\d{2})\b", 1),
        // Pattern 5: XX.XX with transaction context (more specific)
        (r"(?i)(?:for|of|total|amount|payment|charge|transaction|giá trị|giá\s+trị)\s+[^0-9]{0,50}?(\d{1,3}(?:,\d{3})*(?:\.\d{2})?)\b", 1),
        // Pattern 6: XX.XX (generic - last resort)
        (r"(?i)\b(\d{1,3}(?:,\d{3})*(?:\.\d{2})?)\b", 1),
        // Pattern 5: Debit/Credit amount patterns
        (r"(?i)(?:debit|credit|charge)\s*(?:of|amount)?\s*[:=]?\s*[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        // Pattern 6: Transaction for $XX.XX
        (r"(?i)transaction\s+(?:for|of)\s*[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        // Pattern 7: Table format: | Amount | $XX.XX |
        (r"(?i)\|\s*amount\s*\|\s*[$€£¥]?\s*([\d,]+\.?\d{2})\s*\|", 1),
        // Pattern 8: Purchase amount
        (r"(?i)purchase\s+amount\s*[:=]?\s*[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        // Pattern 9: Charged amount
        (r"(?i)(?:charged|billed)\s+(?:an?\s+)?(?:amount\s+of\s+)?[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
        // Pattern 10: Paid amount
        (r"(?i)(?:paid|payment)\s+(?:of|amount)?\s*[:=]?\s*[$€£¥]?\s*([\d,]+\.?\d{2})", 1),
    ];
    
    let mut amount: Option<f64> = None;
    let mut currency = "USD".to_string(); // Default currency
    let mut bank = "Unknown".to_string(); // Default bank
    
    // First, try to detect bank from email
    let body_lower = body.to_lowercase();
    if body_lower.contains("vib") || body_lower.contains("vibvn") || body_lower.contains("vietnam international bank") {
        bank = "VIB".to_string();
    } else if body_lower.contains("chase") {
        bank = "Chase".to_string();
    } else if body_lower.contains("paypal") {
        bank = "PayPal".to_string();
    } else if body_lower.contains("bank of america") || body_lower.contains("bofa") {
        bank = "Bank of America".to_string();
    } else if body_lower.contains("wells fargo") {
        bank = "Wells Fargo".to_string();
    } else if body_lower.contains("capital one") {
        bank = "Capital One".to_string();
    }
    
    // Try to extract amount with currency
    for (pattern, group) in patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(&body) {
                if let Some(amount_str) = caps.get(group) {
                    let cleaned = amount_str.as_str().replace(',', "");
                    if let Ok(parsed_amount) = cleaned.parse::<f64>() {
                        amount = Some(parsed_amount);
                        
                        // Determine currency based on pattern match
                        let full_match = caps.get(0).unwrap().as_str().to_lowercase();
                        if full_match.contains("vnd") {
                            currency = "VND".to_string();
                        } else if full_match.contains("€") || full_match.contains("eur") {
                            currency = "EUR".to_string();
                        } else if full_match.contains("£") || full_match.contains("gbp") {
                            currency = "GBP".to_string();
                        } else if full_match.contains("¥") || full_match.contains("jpy") {
                            currency = "JPY".to_string();
                        } else if full_match.contains("usd") {
                            currency = "USD".to_string();
                        } else if full_match.contains("$") {
                            currency = "USD".to_string(); // $ usually means USD
                        }
                        // If no currency detected but bank is VIB, assume VND
                        else if bank == "VIB" {
                            currency = "VND".to_string();
                        }
                        
                        break;
                    }
                }
            }
        }
    }
    
    // Try to extract transaction ID from VIB bank emails
    let mut transaction_id: Option<String> = None;
    
    // Common patterns for VIB transaction IDs
    let transaction_id_patterns = vec![
        r"Mã\s*giao\s*dịch\s*[:=]\s*([A-Z0-9]+)",
        r"Mã\s*GD\s*[:=]\s*([A-Z0-9]+)",
        r"Transaction\s*ID\s*[:=]\s*([A-Z0-9]+)",
        r"Ref\.\s*([A-Z0-9]+)",
        r"Reference\s*[:=]\s*([A-Z0-9]+)",
        r"ID\s*[:=]\s*([A-Z0-9]+)",
        r"Số\s*giao\s*dịch\s*[:=]\s*([A-Z0-9]+)",
        r"GD\s*([0-9]+)",
        r"#([0-9]+)",
    ];
    
    for pattern in transaction_id_patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(&body) {
                if let Some(id_match) = caps.get(1) {
                    transaction_id = Some(id_match.as_str().to_string());
                    break;
                }
            }
        }
    }
    
    // If no transaction ID found, try to extract from email subject
    if transaction_id.is_none() {
        if let Some(subject) = parsed.headers.get_first_value("Subject") {
            // Look for patterns like "GD123456" in subject
            let subject_patterns = vec![
                r"GD\s*([0-9]+)",
                r"#([0-9]+)",
                r"\[([A-Z0-9]+)\]",
            ];
            
            for pattern in subject_patterns {
                if let Ok(re) = Regex::new(pattern) {
                    if let Some(caps) = re.captures(&subject) {
                        if let Some(id_match) = caps.get(1) {
                            transaction_id = Some(id_match.as_str().to_string());
                            break;
                        }
                    }
                }
            }
        }
    }
    
    let amount = match amount {
        Some(a) => a,
        None => return None,
    };
    
    // Enhanced transaction type detection
    let body_lower = body.to_lowercase();
    let r#type = if body_lower.contains("paid you") ||
                   body_lower.contains("sent you") ||
                   body_lower.contains("credited") || 
                   body_lower.contains("credit") ||
                   body_lower.contains("deposit") ||
                   body_lower.contains("received") ||
                   body_lower.contains("income") ||
                   body_lower.contains("salary") ||
                   body_lower.contains("refund") ||
                   body_lower.contains("reimbursement") ||
                   body_lower.contains("payment received") ||
                   body_lower.contains("incoming") ||
                   body_lower.contains("added to") ||
                   body_lower.contains("money in") ||
                   body_lower.contains("direct deposit") ||
                   body_lower.contains("wire received") ||
                   body_lower.contains("transfer received") ||
                   body_lower.contains("payroll") ||
                   // Vietnamese keywords for incoming money
                   body_lower.contains("nhận được") ||
                   body_lower.contains("đã nhận") ||
                   body_lower.contains("chuyển vào") {
        "in".to_string()
    } else if body_lower.contains("you paid") ||
              body_lower.contains("you sent") ||
              body_lower.contains("debited") || 
              body_lower.contains("debit") ||
              body_lower.contains("withdrawal") ||
              body_lower.contains("purchase") ||
              body_lower.contains("payment") ||
              body_lower.contains("charge") ||
              body_lower.contains("transaction") ||
              body_lower.contains("paid to") ||
              body_lower.contains("sent to") ||
              body_lower.contains("transfer to") ||
              body_lower.contains("withdrawn") ||
              body_lower.contains("declined") ||
              body_lower.contains("authorized") ||
              // Vietnamese keywords for outgoing money
              body_lower.contains("thanh toán") ||
              body_lower.contains("giao dịch") ||
              body_lower.contains("chuyển đi") ||
              body_lower.contains("trừ tiền") {
        "out".to_string()
    } else {
        if amount > 0.0 && (body_lower.contains("refund") || body_lower.contains("reversal")) {
            "in".to_string()
        } else if body_lower.contains("purchase") || body_lower.contains("payment") {
            "out".to_string()
        } else {
            if body_lower.contains('-') && body_lower.contains('$') {
                "out".to_string()
            } else {
                "out".to_string()
            }
        }
    };
    
    // Comprehensive date patterns for bank emails
    let date_patterns = vec![
        // ISO format
        r"(?i)(\d{4}-\d{2}-\d{2})",
        // Standard date formats
        r"(?i)(?:date|transaction\s+date|posted\s+date|trans\s*date)\s*[:=]\s*(\d{1,2}[/-]\d{1,2}[/-]\d{2,4})",
        // Full month name formats
        r"(?i)on\s+(\d{1,2} (?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]* \d{4})",
        r"(?i)dated?\s*[:=]?\s*(\d{1,2} (?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]* \d{4})",
        // Short month formats
        r"(?i)\b(\d{1,2}[/-]\d{1,2}[/-]\d{2,4})\b",
        // US format MM/DD/YYYY
        r"(?i)(\d{1,2}/\d{1,2}/\d{4})",
        // International format DD-MM-YYYY
        r"(?i)(\d{1,2}-\d{1,2}-\d{4})",
        // Month-first (European style)
        r"(?i)(\d{1,2}\.\d{1,2}\.\d{4})",
        // Chase date format
        r"(?i)(?:on|from|posted)\s+(\w+ \d{1,2},? \d{4})",
        // Vietnamese date format with time (e.g., "08:51 03/03/2026")
        r"(?i)(?:\d{2}:\d{2}\s+)?(\d{2}/\d{2}/\d{4})",
        // Relative dates
        r"(?i)\b(today|yesterday)\b",
    ];
    
    let mut date = Local::now().date_naive();
    
    for pattern in date_patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(&body) {
                if let Some(date_str) = caps.get(1) {
                    let date_text = date_str.as_str().to_lowercase();
                    
                    // Handle relative dates
                    if date_text == "today" {
                        date = Local::now().date_naive();
                        break;
                    } else if date_text == "yesterday" {
                        date = Local::now().date_naive() - chrono::Duration::days(1);
                        break;
                    }
                    
                    // Try different date formats
                    let formats = ["%m/%d/%Y", "%d/%m/%Y", "%Y-%m-%d", "%m-%d-%Y", "%d-%m-%Y", "%b %d, %Y", "%B %d, %Y", "%d %b %Y", "%d %B %Y", "%m/%d/%y", "%d/%m/%y"];
                    for fmt in formats {
                        if let Ok(parsed) = NaiveDate::parse_from_str(&date_text, fmt) {
                            date = parsed;
                            break;
                        }
                    }
                }
            }
        }
    }
    
    // Extract description
    let description_patterns = vec![
        // Generic patterns
        r"(?i)description\s*[:=]\s*(.+?)(?:\n|$)",
        r"(?i)merchant\s*[:=]\s*(.+?)(?:\n|$)",
        r"(?i)to\s*[:=]\s*(.+?)(?:\n|$)",
        r"(?i)from\s*[:=]\s*(.+?)(?:\n|$)",
        r"(?i)for\s*[:=]\s*(.+?)(?:\n|$)",
        r"(?i)transaction\s+(?:at|with|to)\s*(.+?)(?:\n|$)",
        // PayPal specific
        r"(?i)Transaction\s+ID:\s*(.+?)(?:\n|$)",
        r"(?i)Sent\s+to:\s*(.+?)(?:\n|$)",
        // Chase specific
        r"(?i)Merchant:\s*(.+?)(?:\n|$)",
        r"(?i)Location:\s*(.+?)(?:\n|$)",
        // Generic purchase description
        r"(?i)purchase\s+(?:at|from)?\s*(.+?)(?:\n|$)",
        r"(?i)paid\s+(?:to|for)\s*(.+?)(?:\n|$)",
        // Card transaction
        r"(?i)Card\s+(?:purchase|transaction)\s+(?:at|with)?\s*(.+?)(?:\n|$)",
        // Venmo/Cash App specific
        r"(?i)note\s*[:=]\s*(.+?)(?:\n|$)",
        r"(?i)memo\s*[:=]\s*(.+?)(?:\n|$)",
        r"(?i)for\s*[:=]\s*(.+?)(?:\n|$)",
        // Vietnamese bank patterns
        r"(?i)tại\s*(.+?)(?:\n|$)",
        r"(?i)merchant\s*[:=]\s*(.+?)(?:\n|$)",
        r"(?i)cửa hàng\s*[:=]\s*(.+?)(?:\n|$)",
        r"(?i)địa điểm\s*[:=]\s*(.+?)(?:\n|$)",
        r"(?i)nơi\s*(?:giao dịch|thanh toán)\s*[:=]\s*(.+?)(?:\n|$)",
    ];
    
    let mut description = if r#type == "in" {
        "Bank Credit".to_string()
    } else {
        "Bank Debit".to_string()
    };
    
    for pattern in description_patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(caps) = re.captures(&body) {
                if let Some(desc) = caps.get(1) {
                    let desc_text = desc.as_str().trim();
                    if !desc_text.is_empty() && desc_text.len() < 200 {
                        description = desc_text.to_string();
                        break;
                    }
                }
            }
        }
    }
    
    // Clean up description
    let description = description
        .replace("\n", " ")
        .replace("\r", " ")
        .replace("  ", " ")
        .trim()
        .to_string();
    
    Some(Transaction::from_email(date, description, amount, currency, r#type, bank, transaction_id, email_message_id))
}

pub fn extract_email_body(parsed_mail: &ParsedMail) -> String {
    let mut best_body = String::new();
    let mut html_body = String::new();
    
    extract_email_body_recursive(parsed_mail, &mut best_body, &mut html_body);
    
    if !best_body.is_empty() {
        best_body
    } else if !html_body.is_empty() {
        html_body
    } else {
        String::new()
    }
}

fn extract_email_body_recursive(parsed_mail: &ParsedMail, best_body: &mut String, html_body: &mut String) {
    let mimetype = &parsed_mail.ctype.mimetype;
    
    if mimetype.starts_with("multipart/") {
        for part in &parsed_mail.subparts {
            extract_email_body_recursive(part, best_body, html_body);
            if !best_body.is_empty() {
                return;
            }
        }
        return;
    }
    
    let content = get_decoded_content(parsed_mail);
    if content.trim().is_empty() {
        return;
    }
    
    match mimetype.as_str() {
        "text/plain" => {
            if best_body.is_empty() {
                *best_body = content;
            }
        }
        "text/html" => {
            if html_body.is_empty() {
                *html_body = convert_html_to_text(&content);
            }
        }
        _ => {}
    }
}

fn get_decoded_content(parsed_mail: &ParsedMail) -> String {
    let encoding = parsed_mail.ctype.params.get("charset")
        .map(|s| s.as_str())
        .unwrap_or("utf-8");
    
    if let Ok(body) = parsed_mail.get_body() {
        let transfer_encoding = parsed_mail.ctype.params.get("Content-Transfer-Encoding")
            .map(|s| s.to_lowercase())
            .unwrap_or_default();
        
        let decoded = match transfer_encoding.as_str() {
            "base64" => {
                decode_base64(&body)
            }
            "quoted-printable" => {
                decode_quoted_printable(&body)
            }
            _ => {
                body.as_bytes().to_vec()
            }
        };
        
        if let Ok(text) = std::str::from_utf8(&decoded) {
            if encoding.eq_ignore_ascii_case("iso-8859-1") || encoding.eq_ignore_ascii_case("windows-1252") {
                return decode_latin1(&decoded);
            }
            return text.to_string();
        }
    }
    
    String::new()
}

fn decode_base64(input: &str) -> Vec<u8> {
    // Remove any trailing .= or other non-base64 characters
    let cleaned = input
        .lines()
        .map(|line| {
            // Split line at .= if present
            if let Some(pos) = line.find(".=") {
                &line[..pos]
            } else {
                line
            }
        })
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("");
    
    base64::engine::general_purpose::STANDARD.decode(&cleaned).unwrap_or_else(|_| input.as_bytes().to_vec())
}

fn decode_quoted_printable(input: &str) -> Vec<u8> {
    let mut result = Vec::new();
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
                            result.push(byte);
                        }
                        continue;
                    }
                }
            }
            result.push(b'=');
        } else {
            result.push(c as u8);
        }
    }
    
    result
}

fn decode_latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| b as char).collect()
}

fn convert_html_to_text(html: &str) -> String {
    let with_newlines = html
        .replace("</p>", "\n")
        .replace("</div>", "\n")
        .replace("</tr>", "\n")
        .replace("</li>", "\n")
        .replace("</h1>", "\n")
        .replace("</h2>", "\n")
        .replace("</h3>", "\n")
        .replace("</h4>", "\n")
        .replace("</h5>", "\n")
        .replace("</h6>", "\n")
        .replace("<br>", "\n")
        .replace("<br/>", "\n")
        .replace("<br />", "\n")
        .replace("\r\n", "\n")
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&mdash;", "—")
        .replace("&ndash;", "–")
        .replace("&rsquo;", "'")
        .replace("&lsquo;", "'")
        .replace("&rdquo;", "\"")
        .replace("&ldquo;", "\"")
        .replace("&euro;", "€")
        .replace("&pound;", "£")
        .replace("&yen;", "¥");
    
    let re = Regex::new(r"<[^>]+>").unwrap();
    let text_only = re.replace_all(&with_newlines, "").to_string();
    
    let re_spaces = Regex::new(r"(?m)^\s+|(?m)\s+$|\s{2,}").unwrap();
    let cleaned = re_spaces.replace_all(&text_only, "").to_string();
    
    cleaned.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

// Example email for testing
pub fn get_example_bank_email() -> String {
    r#"Subject: Transaction Alert - Debit
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
Your Bank"#.to_string()
}

pub fn get_chase_email_sample() -> String {
    r#"Subject: Chase Alert: Debit Card Purchase $42.87
From: alert@chase.com
Date: Mon, 15 Jan 2024 10:30:00 -0500

Chase Alert

You spent $42.87 on your Chase debit card ending in 1234.

Merchant: AMAZON MARKETPLACE
Date: 01/15/2024

If you don't recognize this transaction, please contact us immediately.

Thank you,
Chase"#.to_string()
}

pub fn get_paypal_email_sample() -> String {
    r#"Subject: Payment sent to merchant@example.com
From: service@paypal.com
Date: 15 Jan 2024 14:22:00 -0800

You sent a payment of $125.50 USD

Transaction ID: 5O12345678
Sent to: merchant@example.com
Transaction date: January 15, 2024

Thank you for using PayPal."#.to_string()
}

pub fn get_venmo_email_sample() -> String {
    r#"Subject: Venmo: John paid you $35.00
From: venmo@venmo.com
Date: January 15, 2024

John D. paid you $35.00

Note: Dinner
When: Jan 15, 2024 at 7:30 PM

Venmo"#.to_string()
}

pub fn get_multipart_html_email_sample() -> String {
    r#"Subject: Transaction Alert - Your Bank
From: alerts@bankofamerica.com
Date: Mon, 15 Jan 2024 09:00:00 -0500
MIME-Version: 1.0
Content-Type: multipart/alternative; boundary="----=_Part_123456"

------=_Part_123456
Content-Type: text/plain; charset=utf-8

Bank of America Transaction Alert

A debit transaction has been posted to your account.

Amount: $78.25
Merchant: WALMART SUPERSTORE
Date: 01/15/2024

Thank you for banking with us.

------=_Part_123456
Content-Type: text/html; charset=utf-8

<html>
<body>
<h1>Bank of America Transaction Alert</h1>
<p>A debit transaction has been posted to your account.</p>
<table>
<tr><td><strong>Amount:</strong></td><td>$78.25</td></tr>
<tr><td><strong>Merchant:</strong></td><td>WALMART SUPERSTORE</td></tr>
<tr><td><strong>Date:</strong></td><td>01/15/2024</td></tr>
</table>
<p>Thank you for banking with us.</p>
</body>
</html>

------=_Part_123456--"#.to_string()
}

pub fn get_base64_encoded_email_sample() -> String {
    r#"Subject: Wells Fargo Alert
From: wf@wellsfargo.com
Date: Mon, 15 Jan 2024 08:00:00 -0800
MIME-Version: 1.0
Content-Type: text/plain; charset=utf-8
Content-Transfer-Encoding: base64

V2VsbHMgRmFyZ28gVHJhbnNhY3Rpb24gQWxlcnQKCllvdXIgYWNjb3VudCBoYXMgYmVlbiBkZWJpdGVk
IGZvciAkNTkuOTkuCgpNZXJjaGFudDogV0FMTUFSVApEYXRlOiAwMS8xNS8yMDI0CgpUaGFuayB5b3U=
.=for banking with Wells Fargo."#.to_string()
}

pub fn get_zelle_email_sample() -> String {
    r#"Subject: Zelle - You sent money
From: no-reply@zellepay.com
Date: January 15, 2024

You sent $50.00 via Zelle

Sent to: john@example.com
From: you@example.com
Date: Jan 15, 2024 at 3:45 PM
Transaction ID: ZL123456789

Thank you for using Zelle."#.to_string()
}

pub fn get_cashapp_email_sample() -> String {
    r#"Subject: Cash App Payment
From: cash@square.com
Date: Mon, 15 Jan 2024 12:00:00 -0500

You paid $25.00

Paid to: @merchant
For: Coffee
Date: January 15, 2024 at 12:00 PM PST

Cash App"#.to_string()
}

pub fn get_credit_card_email_sample() -> String {
    r#"Subject: Capital One: Purchase Alert
From: alerts@capitalone.com
Date: January 15, 2024

Capital One Purchase Alert

Your card ending in 5678 was used for a purchase.

Amount: $156.99
Merchant: TARGET STORE #1234
Date: 01/15/2024

Thank you,
Capital One"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_example_email() {
        let email = get_example_bank_email();
        let transaction = parse_transaction_from_email(&email).unwrap();
        
        assert_eq!(transaction.amount, 50.0);
        assert_eq!(transaction.r#type, "out");
        assert_eq!(transaction.description, "GROCERY STORE PURCHASE");
    }
    
    #[test]
    fn test_extract_email_body() {
        let email = "Content-Type: text/plain\r\n\r\nHello World";
        let parsed = parse_mail(email.as_bytes()).unwrap();
        let body = extract_email_body(&parsed);
        
        assert_eq!(body, "Hello World");
    }
    
    #[test]
    fn test_parse_chase_email() {
        let email = get_chase_email_sample();
        let transaction = parse_transaction_from_email(&email);
        assert!(transaction.is_some());
        let t = transaction.unwrap();
        assert_eq!(t.amount, 42.87);
        assert_eq!(t.r#type, "out");
    }
    
    #[test]
    fn test_parse_paypal_email() {
        let email = get_paypal_email_sample();
        let transaction = parse_transaction_from_email(&email);
        assert!(transaction.is_some());
        let t = transaction.unwrap();
        assert_eq!(t.amount, 125.50);
        assert_eq!(t.r#type, "out");
    }
    
    #[test]
    fn test_parse_venmo_email() {
        let email = get_venmo_email_sample();
        let transaction = parse_transaction_from_email(&email);
        assert!(transaction.is_some());
        let t = transaction.unwrap();
        assert_eq!(t.amount, 35.00);
        assert_eq!(t.r#type, "in");
    }
    
    #[test]
    fn test_parse_multipart_html_email() {
        let email = get_multipart_html_email_sample();
        let transaction = parse_transaction_from_email(&email);
        assert!(transaction.is_some());
        let t = transaction.unwrap();
        assert_eq!(t.amount, 78.25);
        assert_eq!(t.r#type, "out");
    }
    
    #[test]
    fn test_parse_zelle_email() {
        let email = get_zelle_email_sample();
        let transaction = parse_transaction_from_email(&email);
        assert!(transaction.is_some());
        let t = transaction.unwrap();
        assert_eq!(t.amount, 50.00);
        assert_eq!(t.r#type, "out");
    }
    
    #[test]
    fn test_parse_cashapp_email() {
        let email = get_cashapp_email_sample();
        let transaction = parse_transaction_from_email(&email);
        assert!(transaction.is_some());
        let t = transaction.unwrap();
        assert_eq!(t.amount, 25.00);
        assert_eq!(t.r#type, "out");
    }
    
    #[test]
    fn test_parse_credit_card_email() {
        let email = get_credit_card_email_sample();
        let transaction = parse_transaction_from_email(&email);
        assert!(transaction.is_some());
        let t = transaction.unwrap();
        assert_eq!(t.amount, 156.99);
        assert_eq!(t.r#type, "out");
    }
    
    #[test]
    fn test_html_to_text_conversion() {
        let html = "<p>Hello<br>World</p><div>Test</div>";
        let text = convert_html_to_text(html);
        assert!(text.contains("Hello"));
        assert!(text.contains("World"));
        assert!(text.contains("Test"));
    }
    
    #[test]
    fn test_base64_decoding() {
        let encoded = "SGVsbG8gV29ybGQ=";
        let decoded = decode_base64(encoded);
        assert_eq!(decoded, "Hello World".as_bytes());
    }
    
    #[test]
    fn test_quoted_printable_decoding() {
        let encoded = "Hello=20World=0AThis is a new line";
        let decoded = decode_quoted_printable(encoded);
        let text = String::from_utf8(decoded).unwrap();
        assert!(text.contains("Hello World"));
        assert!(text.contains("This is a new line"));
    }
}