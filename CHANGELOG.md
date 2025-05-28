# Changelog

All notable changes to the `hns` project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Added `doctor` command to check system health and dependencies
- Integrated checks for network connectivity, Ollama service, and gemma3:4b model
- Added clear status indicators and actionable error messages
- Updated documentation with doctor command usage instructions
### Changed
- Converted `--summarize` flag to a `summarize` subcommand for consistency
- Updated documentation to reflect the new subcommand structure

## [0.3.1] - 2025-05-18
### Added
- Added new `changelog-entry` Makefile target for easier changelog management
- Improved the release process with better Makefile targets
- Fixed formatting issues in documentation
- Reorganized dependency section in README.md

## [0.3.0] - 2025-05-18
### Added 
- Added `--summarize` flag to enable URL content summarization
- Integrated with Ollama API to generate summaries using the gemma3:4b model
- Enhanced README with instructions for using the summarization feature 

## [0.2.0] - 2025-05-17
### Added
- Enhanced GitHub Actions workflow with multi-platform builds
- Automated binary packaging for releases
- Automated publishing to crates.io

## [0.1.0] - 2025-05-17
### Added
- Initial release of Hacker News CLI
- Support for fetching and displaying top stories
- Command-line argument for configuring number of stories
- HTML parsing and proper text formatting
- Special handling for "Show HN" posts

