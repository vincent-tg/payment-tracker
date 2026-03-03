# Email Setup Instructions for Payment Tracker

## Your Configuration
**Email:** `baotg.fin@gmail.com`  
**IMAP Server:** `imap.gmail.com`  
**IMAP Port:** `993`

## Step 1: Get Your Gmail App Password

1. Go to https://myaccount.google.com/
2. Sign in with `baotg.fin@gmail.com`
3. Enable **2-Factor Authentication** if not already enabled
4. Go to **Security** → **App passwords**
5. Generate a new app password for **"Mail"**
6. Copy the 16-character app password

## Step 2: Update Configuration

Edit the configuration file:
```bash
nano ~/.payment-tracker/config.toml
```

Update the password line:
```toml
password = "YOUR_16_CHARACTER_APP_PASSWORD"  # Replace with your actual app password
```

## Step 3: Build and Test the Application

### Option A: Quick Test (Simplified Email)
```bash
cd payment-tracker

# Build the application
source "$HOME/.cargo/env"
cargo build --release

# Initialize database
./target/release/payment-tracker init

# Test basic functionality
./target/release/payment-tracker add --amount 1000 --description "Salary" --type in
./target/release/payment-tracker add --amount 50 --description "Groceries" --type out
./target/release/payment-tracker list
./target/release/payment-tracker summary
```

### Option B: Test Email Fetching (Simplified)
```bash
# The current version has simplified email fetching
# It will show a message about the simplified implementation
./target/release/payment-tracker fetch
```

## Step 4: Verify Email Parsing

The application includes regex-based email parsing. Test with example emails:

```bash
# Create a test email file
cat > test_bank_email.txt << 'EOF'
Subject: Transaction Notification
From: yourbank@example.com
Date: 02/03/2024

Amount: $150.75
Description: ONLINE PAYMENT
Date: 02/03/2024
Type: CREDITED
EOF

# Test parsing (you can create a simple Rust test)
```

## Current Email Implementation Status

### ✅ **Working Features:**
1. **Email configuration** - Properly set up for Gmail
2. **Regex parsing** - Can parse common bank email formats
3. **Transaction extraction** - Extracts amount, date, description, type
4. **Configuration management** - TOML-based config file

### ⚠️ **Simplified Features:**
1. **IMAP fetching** - Currently uses a simplified implementation
2. **Email connectivity** - Needs full IMAP library integration

### 🔧 **To Add Full Email Support:**

Update `Cargo.toml`:
```toml
[dependencies]
# Replace current simplified email with:
imap = "2.4"  # Stable version
mailparse = "0.14"  # For parsing email bodies
native-tls = "0.2"  # For SSL/TLS
```

Then update `src/email.rs` to implement proper IMAP connectivity.

## Testing Your Setup

Run this verification script:
```bash
cd payment-tracker
chmod +x verify_app.sh
source "$HOME/.cargo/env"
./verify_app.sh
```

## Expected Output

When you run `./target/release/payment-tracker fetch` with the current simplified implementation, you'll see:
```
Note: Email fetching is simplified in this version.
To implement full IMAP support, update the email module.
```

## Next Steps for Full Email Integration

1. **Update dependencies** in `Cargo.toml`
2. **Implement IMAP client** in `src/email.rs`
3. **Test with your Gmail account**
4. **Handle different bank email formats**

## Troubleshooting

### Common Issues:

1. **"Invalid credentials" error**
   - Verify 2-factor authentication is enabled
   - Use app password (not your regular password)
   - Ensure app password is 16 characters with no spaces

2. **"IMAP not enabled" error**
   - In Gmail: Settings → See all settings → Forwarding and POP/IMAP
   - Enable IMAP access

3. **"Connection refused" error**
   - Check firewall settings
   - Verify port 993 is open
   - Try with SSL/TLS enabled

4. **Application build errors**
   - Ensure Rust is installed: `rustc --version`
   - Check dependencies: `cargo check`

## Security Notes

1. **Never commit** your app password to version control
2. **Use environment variables** for production deployment
3. **Regularly rotate** app passwords
4. **Monitor** your Google account security page

## Support

If you encounter issues:
1. Check Google Account security settings
2. Verify app password generation
3. Test with the simplified implementation first
4. Review application logs

Your payment tracker is now configured for `baotg.fin@gmail.com` and ready for testing! 🚀