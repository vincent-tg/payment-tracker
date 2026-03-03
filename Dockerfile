# Multi-stage build for Rust application
FROM rust:1.73-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static

# Create app directory
WORKDIR /app

# Copy source files
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates openssl libgcc sqlite

# Create app user
RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/payment-tracker /app/payment-tracker

# Copy configuration template
COPY .env.example /app/.env.example

# Create data directory for SQLite database
RUN mkdir -p /app/data && chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose port (if needed for future web interface)
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_PATH=/app/data/payments.db

# Create entrypoint script
RUN echo '#!/bin/sh' > /app/entrypoint.sh && \
    echo 'if [ ! -f "$DATABASE_PATH" ]; then' >> /app/entrypoint.sh && \
    echo '  echo "Initializing database..."' >> /app/entrypoint.sh && \
    echo '  /app/payment-tracker init' >> /app/entrypoint.sh && \
    echo 'fi' >> /app/entrypoint.sh && \
    echo 'exec "$@"' >> /app/entrypoint.sh && \
    chmod +x /app/entrypoint.sh

# Set entrypoint
ENTRYPOINT ["/app/entrypoint.sh"]

# Default command
CMD ["/app/payment-tracker", "--help"]