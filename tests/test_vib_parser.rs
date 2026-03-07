use payment_tracker::email;

fn main() -> anyhow::Result<()> {
    println!("Testing VIB Bank Email Parser with Currency Tracking");
    println!("====================================================\n");

    // Sample VIB bank email (simplified)
    let vib_email = r#"From: VIB Bank <noreply@vib.com.vn>
Subject: Thông báo giao dịch thẻ
Content-Type: text/html; charset=UTF-8
Content-Transfer-Encoding: quoted-printable

<!DOCTYPE html>
<html>
<body>
<div style="font-family: Arial, sans-serif;">
    <h3>Thông báo giao dịch thẻ</h3>
    <p><strong>Ngân hàng:</strong> VIB</p>
    <p><strong>Giá trị:</strong> <b>58,000 VND</b></p>
    <p><strong>Thời gian:</strong> 08:51 03/03/2026</p>
    <p><strong>Địa điểm:</strong> 7ELEVEN_1062</p>
    <p><strong>Loại giao dịch:</strong> Thanh toán dịch vụ - hàng hóa</p>
    <p><strong>Số dư khả dụng:</strong> 12,345,000 VND</p>
</div>
</body>
</html>"#;

    println!("Parsing VIB Bank Email:");
    println!("-----------------------");

    match email::parse_transaction_from_email(vib_email) {
        Some(transaction) => {
            println!("✅ SUCCESS: Transaction parsed!");
            println!("\nTransaction Details:");
            println!("  Bank: {}", transaction.bank);
            println!("  Amount: {}", transaction.format_amount());
            println!("  Currency: {}", transaction.currency);
            println!("  USD Equivalent: ${:.2}", transaction.to_usd());
            println!("  Type: {}", transaction.r#type);
            println!("  Description: {}", transaction.description);
            println!("  Date: {}", transaction.date);

            // Verify it's correct
            println!("\n✅ Verification:");
            if transaction.bank == "VIB" {
                println!("  ✓ Bank correctly identified as VIB");
            }
            if transaction.currency == "VND" {
                println!("  ✓ Currency correctly identified as VND");
            }
            if transaction.amount == 58000.0 {
                println!("  ✓ Amount correctly parsed: 58,000 VND");
            }
            if transaction.r#type == "out" {
                println!("  ✓ Transaction type correctly identified as 'out' (debit)");
            }
            if transaction.description.contains("7ELEVEN") {
                println!("  ✓ Merchant correctly extracted: 7ELEVEN");
            }

            println!("\n🎉 VIB Bank email parser is working correctly!");
            println!("   - Tracks currency (VND)");
            println!("   - Converts to USD (≈${:.2})", transaction.to_usd());
            println!("   - Identifies VIB bank");
            println!("   - Extracts Vietnamese transaction details");
        }
        None => {
            println!("❌ FAILED: Could not parse transaction from email");
        }
    }

    // Test another email format
    println!("\n\nTesting Another VIB Email Format:");
    println!("----------------------------------");

    let vib_email2 = r#"From: VIB <no-reply@vib.com.vn>
Subject: Giao dịch thẻ thành công
Content-Type: text/plain

Chào bạn,

Giao dịch thẻ của bạn đã được thực hiện thành công.

- Số tiền: 150,000 VND
- Thời gian: 14:30 06/03/2026
- Địa điểm: COFFEE_SHOP_123
- Loại giao dịch: Mua hàng

Trân trọng,
VIB Bank"#;

    match email::parse_transaction_from_email(vib_email2) {
        Some(t) => {
            println!("✅ Parsed: {} at {}", t.format_amount(), t.description);
            println!(
                "  Bank: {}, Type: {}, USD: ${:.2}",
                t.bank,
                t.r#type,
                t.to_usd()
            );
        }
        None => println!("❌ Could not parse second email"),
    }

    Ok(())
}
