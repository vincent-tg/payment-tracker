# Email Pattern Analysis for Payment Tracker

## Current Status
- **8 out of 9** sample emails parse successfully
- **1 failing**: Wells Fargo (Base64 encoded)
- **1 incorrect type**: Venmo (should be "in" not "out")

## Pattern Analysis

### 1. Amount Extraction Patterns (Working)
- `Amount: $XX.XX` ✓
- `$XX.XX` (standalone) ✓
- `USD XX.XX` ✓
- `You spent $XX.XX` ✓
- `You sent $XX.XX` ✓
- `You paid $XX.XX` ✓

### 2. Date Extraction Patterns (Mostly Working)
- `Date: 01/15/2024` ✓
- `Date: January 15, 2024` ✓
- `When: Jan 15, 2024 at 7:30 PM` ✓
- `Transaction date: January 15, 2024` ✓

### 3. Description/Merchant Patterns
- `Merchant: NAME` ✓
- `Description: TEXT` ✓
- `Sent to: EMAIL` ✓
- `Paid to: @USER` ✓
- `For: NOTE` ✓
- `Note: TEXT` (Venmo - not captured) ✗

### 4. Transaction Type Detection Issues

**Current logic problems:**
1. **Venmo**: "John paid you $35.00" → Should be `in` but detected as `out`
   - Issue: "paid" keyword triggers "out" but "paid you" means "in"
   
2. **Type detection keywords need refinement:**
   - "paid you" → `in` (money received)
   - "you paid" → `out` (money sent)
   - "sent to" → `out`
   - "received from" → `in`

### 5. Base64 Email Issue (Wells Fargo)
The email has:
```
Content-Transfer-Encoding: base64

V2VsbHMgRmFyZ28gVHJhbnNhY3Rpb24gQWxlcnQKCllvdXIgYWNjb3VudCBoYXMgYmVlbiBkZWJpdGVk
IGZvciAkNTkuOTkuCgpNZXJjaGFudDogV0FMTUFSVApEYXRlOiAwMS8xNS8yMDI0CgpUaGFuayB5b3U=
.=for banking with Wells Fargo.
```

**Issues:**
1. The base64 string has a `.=` at the end which might break decoding
2. There's extra text after the base64 section

## Recommended Improvements

### 1. Fix Transaction Type Detection
```rust
// Current logic needs to check for "paid you" vs "you paid"
let body_lower = body.to_lowercase();
let r#type = if body_lower.contains("paid you") || 
               body_lower.contains("sent you") ||
               body_lower.contains("credited to you") {
    "in".to_string()
} else if body_lower.contains("you paid") ||
          body_lower.contains("you sent") ||
          body_lower.contains("debited from") {
    "out".to_string()
} else {
    // Existing logic...
};
```

### 2. Improve Base64 Decoding
```rust
fn decode_base64(input: &str) -> Vec<u8> {
    let cleaned = input
        .lines()
        .take_while(|line| !line.starts_with(".=")) // Stop at .= boundary
        .collect::<Vec<_>>()
        .join("");
    
    base64::engine::general_purpose::STANDARD
        .decode(&cleaned)
        .unwrap_or_else(|_| input.as_bytes().to_vec())
}
```

### 3. Add Note Field Extraction (Venmo)
```rust
// Add to description patterns
let note_patterns = vec![
    r"(?i)note\s*[:=]\s*(.+?)(?:\n|$)",
    r"(?i)for\s*[:=]\s*(.+?)(?:\n|$)",
    r"(?i)memo\s*[:=]\s*(.+?)(?:\n|$)",
];
```

### 4. Enhance Date Parsing
Add more date formats:
- `Jan 15, 2024 at 7:30 PM`
- `15 Jan 2024`
- `2024-01-15T14:22:00-0800`

### 5. Bank-Specific Patterns
Consider adding bank-specific parsers:
- Chase: Looks for "Chase Alert" and card ending patterns
- PayPal: Transaction ID patterns
- Venmo: "@username" patterns
- Zelle: "via Zelle" indicator

## Testing Strategy
1. Create a corpus of real bank emails (anonymized)
2. Add validation for each bank's format
3. Implement confidence scoring for parsing
4. Log parsing failures for continuous improvement

## Next Steps
1. Fix Venmo transaction type detection
2. Fix Base64 email parsing
3. Add note field extraction
4. Test with more real-world email samples
5. Consider ML-based parsing for complex formats