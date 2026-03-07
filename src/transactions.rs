pub fn categorize_transaction(description: &str) -> String {
    let desc_lower = description.to_lowercase();

    if desc_lower.contains("salary") || desc_lower.contains("payroll") {
        "Salary".to_string()
    } else if desc_lower.contains("grocery") || desc_lower.contains("supermarket") {
        "Groceries".to_string()
    } else if desc_lower.contains("restaurant")
        || desc_lower.contains("cafe")
        || desc_lower.contains("food")
    {
        "Dining".to_string()
    } else if desc_lower.contains("fuel")
        || desc_lower.contains("gas")
        || desc_lower.contains("petrol")
    {
        "Transport".to_string()
    } else if desc_lower.contains("electricity")
        || desc_lower.contains("water")
        || desc_lower.contains("utility")
    {
        "Utilities".to_string()
    } else if desc_lower.contains("rent") || desc_lower.contains("mortgage") {
        "Housing".to_string()
    } else if desc_lower.contains("internet")
        || desc_lower.contains("phone")
        || desc_lower.contains("mobile")
    {
        "Communication".to_string()
    } else if desc_lower.contains("entertainment")
        || desc_lower.contains("movie")
        || desc_lower.contains("streaming")
    {
        "Entertainment".to_string()
    } else {
        "Other".to_string()
    }
}

pub fn validate_transaction_type(r#type: &str) -> bool {
    r#type == "in" || r#type == "out"
}

pub fn format_amount(amount: f64) -> String {
    format!("${:.2}", amount)
}
