# Contributing to Rust Modular Projects

Welcome to the Rust Modular Projects! We're excited that you're interested in contributing. Before you get started, please take a moment to read through this guide.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Pull Request Process](#pull-request-process)
- [Testing Guidelines](#testing-guidelines)
- [Security](#security)

## Code of Conduct

This project adheres to the Rust Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to [md.abir1203@gmail.com].

## Getting Started

1. **Fork the Repository**
   ```bash
   git clone https://github.com/YOUR-USERNAME/rust-modular-projects.git
   cd rust-modular-projects
   git remote add upstream https://github.com/mdabir1203/rust-modular-projects.git
   ```

2. **Set Up Development Environment**
   - Install Rust via rustup: https://rustup.rs/
   - Required dependencies:
     ```bash
     rustup component add rustfmt
     rustup component add clippy
     cargo install cargo-audit
     ```

3. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

1. **Keep Your Fork Updated**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run Tests Locally**
   ```bash
   cargo test
   cargo clippy
   cargo fmt --all -- --check
   ```

3. **Commit Guidelines**
   - Use conventional commits format:
     - feat: New feature
     - fix: Bug fix
     - docs: Documentation changes
     - style: Code style changes
     - refactor: Code refactoring
     - test: Test updates
     - chore: Maintenance tasks

   Example:
   ```
   feat(auth): implement JWT authentication
   
   - Add JWT token generation
   - Implement token validation
   - Add unit tests for auth module
   ```

## Pull Request Process

1. **Before Submitting**
   - Update documentation if needed
   - Add tests for new features
   - Ensure all tests pass
   - Update CHANGELOG.md if applicable

2. **PR Template**
   ```markdown
   ## Description
   [Describe your changes]

   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update

   ## Testing
   - [ ] Tests added/updated
   - [ ] All tests passing

   ## Screenshots (if applicable)
   ```

## Coding Standards

1. **Rust Style Guidelines**
   - Follow Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
   - Use `rustfmt` for consistent formatting
   - Run `clippy` and address all warnings


3. **Documentation**
   - All public items must have doc comments
   - Include examples in doc comments
   - Use proper Rust documentation syntax
   ```rust
   /// Performs an operation.
   ///
   /// # Examples
   ///
   /// ```
   /// let result = my_function(42);
   /// assert_eq!(result, 84);
   /// ```
   pub fn my_function(input: i32) -> i32 {
       // Implementation
   }
   ```

## Testing Guidelines

1. **Unit Tests**
   - Write tests for all public functions
   - Use descriptive test names
   - Follow arrange-act-assert pattern

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_function_expected_behavior() {
           // Arrange
           let input = 42;

           // Act
           let result = my_function(input);

           // Assert
           assert_eq!(result, expected_value);
       }
   }
   ```

2. **Integration Tests**
   - Place in `tests/` directory
   - Test module interactions
   - Include error cases

## Documentation

1. **API Documentation**
   - Use `cargo doc` to generate documentation
   - Include examples
   - Document error conditions
   - Add links to related items

2. **README Updates**
   - Keep installation instructions current
   - Update feature list
   - Maintain troubleshooting section

## Security

1. **Reporting Security Issues**
   - Email security concerns to [md.abir1203@gmail.com]
   - Do not create public issues for security vulnerabilities

2. **Security Best Practices**
   - Use `cargo audit` regularly
   - Keep dependencies updated
   - Follow Rust security guidelines
   - Avoid unsafe code unless absolutely necessary

## Support

Need help? You can:
- Open an issue
- Email the maintainers

Thank you for contributing to Rust Modular Projects! 🦀
