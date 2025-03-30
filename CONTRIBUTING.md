# Contributing to Wrale Agnostic Content Dependency Manager

Thank you for your interest in contributing to `wrale-acdm`! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

This project adheres to a Code of Conduct that all contributors are expected to follow. Please read [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before contributing.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with a clear description:

1. What you were trying to do
2. What you expected to happen
3. What actually happened
4. Steps to reproduce the issue
5. Your environment (OS, Rust version, Git version)

### Suggesting Enhancements

For feature requests or enhancements:

1. Open an issue with the "enhancement" label
2. Describe the feature you'd like to see
3. Explain why it would be useful
4. Suggest an implementation approach if possible

### Pull Requests

1. Fork the repository
2. Create a branch for your changes
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass with `cargo test`
6. Update documentation if needed
7. Submit a pull request

## Development Process

### Setting Up Development Environment

1. Install Rust (1.70+ recommended)
2. Clone the repository
3. Build the project: `cargo build`
4. Run tests: `cargo test`

### Project Structure

The project follows clean architecture principles:

- `src/domain`: Core business logic and entities
- `src/application`: Use cases and application-specific business rules
- `src/infrastructure`: External implementations (Git, file system, etc.)
- `src/interfaces`: Adapters between the domain and external systems
- `src/cli`: Command-line interface

### Coding Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use descriptive variable and function names
- Add comments for complex logic
- Write comprehensive tests

### Testing

- Write unit tests for new functionality
- Ensure integration tests cover new features
- Run `cargo clippy` to check for common mistakes
- Run `cargo fmt` to ensure consistent formatting

## Release Process

1. Version bump in Cargo.toml
2. Update CHANGELOG.md
3. Create a release branch
4. Submit a pull request to main
5. Tag the release

## License

By contributing to this project, you agree that your contributions will be licensed under the project's [MIT License](LICENSE).

---

Copyright (c) 2025 Wrale LTD <contact@wrale.com>
