# ry26

[![Rust CI](https://github.com/bounverif/ry26/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/bounverif/ry26/actions/workflows/rust-ci.yml)
[![Code Coverage](https://github.com/bounverif/ry26/actions/workflows/coverage.yml/badge.svg)](https://github.com/bounverif/ry26/actions/workflows/coverage.yml)

## CI/CD

This repository includes GitHub Actions workflows for continuous integration:

- **Rust CI** (`rust-ci.yml`): Runs tests across multiple platforms (Ubuntu, Windows, macOS) and Rust versions (stable, beta, nightly), along with formatting (rustfmt) and linting (clippy) checks.
- **Code Coverage** (`coverage.yml`): Generates code coverage reports using cargo-llvm-cov and uploads them to Codecov.