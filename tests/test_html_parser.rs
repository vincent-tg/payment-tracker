use payment_tracker::email;

fn main() {
    // Test 1: Plain text email
    println!("=== Test 1: Plain Text Email ===");
    let plain_email = std::fs::read_to_string("test_email.txt").unwrap();
    match email::parse_transaction_from_email(&plain_email) {
        Some(t) => println!("Parsed: ${} - {} - {}", t.amount, t.r#type, t.description),
        None => println!("Failed to parse plain email"),
    }

    // Test 2: HTML email
    println!("\n=== Test 2: HTML Email ===");
    let html_email = std::fs::read_to_string("test_html_email.txt").unwrap();
    match email::parse_transaction_from_email(&html_email) {
        Some(t) => println!("Parsed: ${} - {} - {}", t.amount, t.r#type, t.description),
        None => println!("Failed to parse HTML email"),
    }

    // Test 3: Example bank email from code
    println!("\n=== Test 3: Example Bank Email ===");
    let example_email = email::get_example_bank_email();
    match email::parse_transaction_from_email(&example_email) {
        Some(t) => println!("Parsed: ${} - {} - {}", t.amount, t.r#type, t.description),
        None => println!("Failed to parse example email"),
    }
}
