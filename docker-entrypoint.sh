#!/bin/sh
set -e

# Docker entrypoint for payment-tracker
# Handles initialization and configuration before starting the application

echo "🚀 Payment Tracker starting..."

# If config file doesn't exist, copy the example
if [ ! -f /app/config.toml ]; then
    if [ -f /app/config_example.toml ]; then
        echo "📋 No config.toml found, copying example config..."
        cp /app/config_example.toml /app/config.toml
    fi
fi

# Initialize database if running serve or daily commands
case "${1}" in
    /app/payment-tracker)
        shift
        case "${1}" in
            serve|api|daily|fetch|init)
                echo "📦 Initializing database..."
                /app/payment-tracker init || echo "⚠️  Database init skipped (may already exist)"
                ;;
        esac
        exec /app/payment-tracker "$@"
        ;;
    *)
        exec "$@"
        ;;
esac
