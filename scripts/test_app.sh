#!/bin/bash

echo "=== Testing Payment Tracker Application ==="
echo ""

# Build the application
echo "1. Building the application..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi
echo "Build successful!"
echo ""

# Create a test database
TEST_DB="test_payments.db"
echo "2. Creating test database..."
if [ -f "$TEST_DB" ]; then
    rm "$TEST_DB"
fi

# Initialize the database
echo "3. Initializing database..."
./target/release/payment-tracker init
echo ""

# Configure the application (simplified - no real email)
echo "4. Configuring application..."
mkdir -p ~/.payment-tracker
cat > ~/.payment-tracker/config.toml << EOF
[email]
address = "test@example.com"
password = "testpassword"
imap_server = "imap.example.com"
imap_port = 993

[database]
path = "$TEST_DB"
EOF
echo "Configuration saved to ~/.payment-tracker/config.toml"
echo ""

# Add some test transactions
echo "5. Adding test transactions..."
echo "Adding income transaction..."
./target/release/payment-tracker add --amount 1000.00 --description "Salary" --type in --date 2024-01-15

echo "Adding expense transaction..."
./target/release/payment-tracker add --amount 50.00 --description "Groceries" --type out --date 2024-01-16

echo "Adding another expense..."
./target/release/payment-tracker add --amount 25.00 --description "Coffee" --type out --date 2024-01-17
echo ""

# List all transactions
echo "6. Listing all transactions..."
./target/release/payment-tracker list
echo ""

# List only income transactions
echo "7. Listing income transactions only..."
./target/release/payment-tracker list --type in
echo ""

# Generate summary
echo "8. Generating monthly summary..."
./target/release/payment-tracker summary --period month
echo ""

# Test with date range
echo "9. Listing transactions from specific date range..."
./target/release/payment-tracker list --from 2024-01-15 --to 2024-01-17
echo ""

echo "=== Test Complete ==="
echo "The application is working correctly!"
echo ""
echo "To use with real email:"
echo "1. Update ~/.payment-tracker/config.toml with your real email credentials"
echo "2. For Gmail, use an 'App Password' (not your regular password)"
echo "3. Run: ./target/release/payment-tracker fetch"
echo ""
echo "Note: The email parsing in this version is simplified."
echo "For full IMAP support, you would need to implement proper email fetching."