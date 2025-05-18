# Contributing to hns

Thank you for your interest in contributing to the `hns` project! This document provides guidelines and instructions for contributing.

## Code of Conduct

Please be respectful and considerate when interacting with other contributors. Treat everyone with respect and empathy.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/hns.git`
3. Set up the original repository as upstream: `git remote add upstream https://github.com/original-owner/hns.git`
4. Create a branch for your changes: `git checkout -b feature/your-feature-name`

## Development Workflow

This project includes a Makefile to simplify common development tasks:

- `make build` - Build the binary
- `make test` - Run tests
- `make fmt` - Format code
- `make lint` - Lint the code
- `make help` - Show all available commands

## Pull Request Process

1. Update the CHANGELOG.md with details of your changes.
2. Ensure all tests pass by running `make test`.
3. Format your code with `make fmt`.
4. Lint your code with `make lint`.
5. Submit your pull request against the `main` branch.
6. Ensure the PR description clearly describes the problem and solution.

## Coding Standards

- Follow Rust's official style guide. Run `cargo fmt` before committing.
- Write comprehensive documentation for public APIs.
- Include tests for new functionality.
- Keep functions focused and small.

## Reporting Issues

If you find a bug or have a suggestion for improvement:

1. Check if the issue already exists in the GitHub issue tracker.
2. If not, create a new issue with a clear title and description.
3. Include steps to reproduce the issue and the expected behavior.
4. Mention your environment (OS, Rust version, etc.).

## Feature Requests

Feature requests are welcome. To submit a feature request:

1. Describe the feature you'd like to see added.
2. Explain why this feature would be useful to most users.
3. Provide examples of how the feature would work.

## License

By contributing to this project, you agree that your contributions will be licensed under the project's BSD 3-Clause license.
