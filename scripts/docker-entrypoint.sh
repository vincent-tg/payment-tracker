#!/bin/sh
set -e

echo "🚀 Starting VIB Bank Payment Tracker"
echo "===================================="

# Check if config file exists, create from example if not
if [ ! -f "/app/config.toml" ]; then
    echo "📝 Config file not found, creating from example..."
    cp /app/config_example.toml /app/config.toml
    echo "⚠️  Please update /app/config.toml with your credentials"
fi

# Initialize database if needed
if [ "$1" = "init-db" ]; then
    echo "🗄️  Initializing database..."
    /app/payment-tracker init-db
    exit 0
fi

# Run daily tracker
if [ "$1" = "daily" ]; then
    echo "📅 Running daily VIB bank tracking..."
    /app/payment-tracker daily
    exit 0
fi

# Default: run the application
echo "🔄 Starting payment tracker..."
exec "$@"