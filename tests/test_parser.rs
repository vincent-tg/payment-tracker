use payment_tracker::email;

fn main() {
    let email_text = std::fs::read_to_string("test_email.txt").unwrap();
    
    match email::parse_transaction_from_email(&email_text) {
        Some(transaction) => {
            println!("Successfully parsed transaction!");
            println!("Amount: ${}", transaction.amount);
            println!("Type: {}", transaction.r#type);
            println!("Description: {}", transaction.description);
            println!("Date: {}", transaction.date);
        }
        None => {
            println!("Failed to parse transaction from email");
        }
    }
}