use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExchangeRate {
    pub base: String,
    pub rates: HashMap<String, f64>,
}

pub struct CurrencyConverter {
    rates: HashMap<String, f64>,
}

impl CurrencyConverter {
    pub fn new() -> Self {
        let mut rates = HashMap::new();
        
        // Default exchange rates (approximate)
        // 1 USD = 23,000 VND (Vietnamese Dong)
        // 1 USD = 0.92 EUR
        // 1 USD = 0.79 GBP
        // 1 USD = 150 JPY
        
        rates.insert("USD".to_string(), 1.0);
        rates.insert("VND".to_string(), 23000.0);
        rates.insert("EUR".to_string(), 0.92);
        rates.insert("GBP".to_string(), 0.79);
        rates.insert("JPY".to_string(), 150.0);
        rates.insert("AUD".to_string(), 1.52);
        rates.insert("CAD".to_string(), 1.36);
        
        Self { rates }
    }
    
    pub fn convert(&self, amount: f64, from_currency: &str, to_currency: &str) -> Result<f64> {
        let from_rate = self.rates.get(from_currency.to_uppercase().as_str())
            .ok_or_else(|| anyhow::anyhow!("Unknown currency: {}", from_currency))?;
        
        let to_rate = self.rates.get(to_currency.to_uppercase().as_str())
            .ok_or_else(|| anyhow::anyhow!("Unknown currency: {}", to_currency))?;
        
        // Convert to USD first, then to target currency
        let usd_amount = amount / from_rate;
        let target_amount = usd_amount * to_rate;
        
        Ok(target_amount)
    }
    
    pub fn convert_to_usd(&self, amount: f64, from_currency: &str) -> Result<f64> {
        self.convert(amount, from_currency, "USD")
    }
    
    pub fn format_amount(&self, amount: f64, currency: &str) -> String {
        match currency.to_uppercase().as_str() {
            "VND" => format!("{:.0} VND", amount),
            "JPY" => format!("{:.0} JPY", amount),
            _ => format!("${:.2} {}", amount, currency),
        }
    }
    
    pub fn get_supported_currencies(&self) -> Vec<&str> {
        self.rates.keys().map(|k| k.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vnd_to_usd_conversion() {
        let converter = CurrencyConverter::new();
        let result = converter.convert(58000.0, "VND", "USD").unwrap();
        // 58,000 VND ≈ 2.52 USD (at 23,000 VND/USD)
        assert!((result - 2.52).abs() < 0.1);
    }
    
    #[test]
    fn test_usd_to_vnd_conversion() {
        let converter = CurrencyConverter::new();
        let result = converter.convert(100.0, "USD", "VND").unwrap();
        // 100 USD ≈ 2,300,000 VND
        assert!((result - 2300000.0).abs() < 1000.0);
    }
    
    #[test]
    fn test_format_vnd() {
        let converter = CurrencyConverter::new();
        let formatted = converter.format_amount(58000.0, "VND");
        assert_eq!(formatted, "58000 VND");
    }
    
    #[test]
    fn test_format_usd() {
        let converter = CurrencyConverter::new();
        let formatted = converter.format_amount(1234.56, "USD");
        assert_eq!(formatted, "$1234.56 USD");
    }
}