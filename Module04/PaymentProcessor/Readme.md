# Payment Processor

This project demonstrates a payment processing system implemented in Rust, showcasing the Factory design pattern and real-world business logic.

## Features

- Support for multiple payment methods (Credit Card, PayPal)
- Factory pattern for creating payment objects
- Payment processing and refund functionality
- Error handling and input validation

## Getting Started

### Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/payment_processor.git
   cd payment_processor
   ```

2. Build the project:
   ```
   cargo build
   ```

### Usage

Run the project with:

```
cargo run
```

This will execute the main function, demonstrating various payment scenarios.

## Project Structure

- `src/main.rs`: Entry point of the application
- `src/lib.rs`: Library root, exposing the public API
- `src/payment/`: Contains payment method implementations
- `src/factory.rs`: Implements the PaymentFactory
- `src/processor.rs`: Implements the PaymentProcessor

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.