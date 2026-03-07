# Multi-stage build for Rust application
FROM rust:1.85-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig perl make

# Create app directory
WORKDIR /app

# Copy source files
COPY . .

# Build the application using BuildKit cache mounts
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp /app/target/release/payment-tracker /tmp/payment-tracker

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Create non-root user
RUN addgroup -g 1000 payment && \
    adduser -D -u 1000 -G payment payment

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /tmp/payment-tracker /app/payment-tracker
COPY --from=builder /app/configs/config_example.toml /app/config_example.toml

# Copy entrypoint script
COPY docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

# Set ownership
RUN chown -R payment:payment /app

# Switch to non-root user
USER payment

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Entrypoint
ENTRYPOINT ["/app/docker-entrypoint.sh"]

# Default command
CMD ["/app/payment-tracker", "serve"]