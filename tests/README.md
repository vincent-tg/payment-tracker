# Tests

This directory contains test files for the payment-tracker application.

## Test Structure

### Unit Tests
Unit tests are located in the `src/` directory alongside the code they test:
- `src/currency.rs` contains currency conversion tests
- `src/email.rs` contains email parsing tests
- Each module has its own `#[cfg(test)]` section

### Integration Tests
Integration tests are located in this directory:
- `tests/integration/` - Tests that require external resources
- `tests/unit/` - Additional unit test files

## Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run specific test module
cargo test currency

# Run with verbose output
cargo test -- --nocapture

# Run integration tests
cargo test --test '*'
```

## Test Files

### Unit Test Files
- `debug_base64.rs` - Base64 decoding tests
- `debug_currency.rs` - Currency debugging tests
- `decode_email.rs` - Email decoding tests
- `simple_currency_test.rs` - Basic currency tests
- `simple_demo.rs` - Demonstration tests
- `simple_test.rs` - Basic functionality tests
- `test_*.rs` - Various component tests

### Integration Test Files
- `integration/test_postgres_integration.rs` - PostgreSQL database tests
- `integration/real_email_test.rs` - Real email parsing tests

## Writing Tests

### Unit Test Example
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```

### Integration Test Example
Create a file in `tests/` directory:
```rust
use payment_tracker;

#[test]
fn test_integration() {
    // Test code here
}
```

## Test Data

Test email files are located in `emails/` directory:
- `real_email_1.eml` - Sample VIB bank email
- `real_email_2.eml` - Sample VIB bank email  
- `real_email_3.eml` - Sample VIB bank email
- `test_email.txt` - Simple test email
- `test_html_email.txt` - HTML email test

## Test Coverage

To generate test coverage reports:
```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

## Best Practices

1. **Isolate tests**: Each test should be independent
2. **Use descriptive names**: Test names should describe what they test
3. **Test edge cases**: Include boundary conditions and error cases
4. **Mock external dependencies**: Use mocks for email and database in unit tests
5. **Clean up**: Tests should clean up any resources they create