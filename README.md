# VIB Payment Tracker

A Rust application for tracking payment cash in/out by parsing bank emails, with a focus on VIB (Vietnam International Bank) transactions.

## Features

- **Email Parsing**: Automatically fetch and parse transaction emails from VIB bank
- **Multi-Currency Support**: Track transactions in VND, USD, and other currencies
- **PostgreSQL Storage**: Store transactions in a PostgreSQL database
- **REST API**: Health endpoints and transaction API via Axum web server
- **CI/CD**: Automated testing and deployment via GitHub Actions

## CI/CD Status
Last test trigger: 2026-03-07 06:33:55 UTC
Fixed: Added PostgreSQL setup for CI tests