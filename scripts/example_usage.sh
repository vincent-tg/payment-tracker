#!/bin/bash

# Example usage script for Payment Tracker

echo "=== Payment Tracker Example Usage ==="
echo ""

# 1. First, configure the application
echo "1. Configuring the application..."
echo "   payment-tracker config \\"
echo "     --email your-email@gmail.com \\"
echo "     --password your-app-password \\"
echo "     --imap-server imap.gmail.com \\"
echo "     --imap-port 993 \\"
echo "     --database payments.db"
echo ""

# 2. Initialize the database
echo "2. Initializing the database..."
echo "   payment-tracker init"
echo ""

# 3. Fetch emails (this would connect to your email server)
echo "3. Fetching and processing emails..."
echo "   payment-tracker fetch"
echo "   # This will:"
echo "   # - Connect to your email server"
echo "   # - Fetch unread emails from last 7 days"
echo "   # - Parse bank transaction emails"
echo "   # - Store transactions in database"
echo ""

# 4. List transactions
echo "4. Listing transactions..."
echo "   payment-tracker list"
echo "   # Shows all transactions in a nice table"
echo ""
echo "   payment-tracker list --type in --limit 5"
echo "   # Shows last 5 income transactions"
echo ""

# 5. Generate summary
echo "5. Generating summary..."
echo "   payment-tracker summary"
echo "   # Monthly summary (default)"
echo ""
echo "   payment-tracker summary --period week"
echo "   # Weekly summary"
echo ""

# 6. Add manual transaction
echo "6. Adding manual transaction..."
echo "   payment-tracker add \\"
echo "     --amount 1000.00 \\"
echo "     --description \"Salary\" \\"
echo "     --type in"
echo ""
echo "   payment-tracker add \\"
echo "     --amount 50.00 \\"
echo "     --description \"Groceries\" \\"
echo "     --type out \\"
echo "     --date 2024-01-15"
echo ""

echo "=== Sample Bank Email Format ==="
echo ""
echo "The application looks for patterns like these in emails:"
echo ""
echo "Subject: Transaction Alert"
echo "Date: 15/01/2024"
echo ""
echo "Dear Customer,"
echo ""
echo "A transaction has been made on your account:"
echo "Amount: $50.00"
echo "Description: GROCERY STORE"
echo "Date: 15/01/2024"
echo "Type: DEBITED"
echo ""
echo "Thank you,"
echo "Your Bank"
echo ""
echo "=== Notes ==="
echo "- For Gmail, use 'App Password' not your regular password"
echo "- The app searches for emails from last 7 days"
echo "- Transactions are deduplicated (same date, description, amount, type)"
echo "- Database is SQLite, stored at the path you configure"