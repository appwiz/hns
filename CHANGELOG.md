# Changelog

All notable changes to the `hns` project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Restructured CLI to use subcommands: `show`, `summarize`, and `doctor`
- Added `doctor` command for running diagnostic checks
- Added network connectivity checks for Hacker News API and Ollama endpoints
- Added Ollama model verification checks
- Improved CLI help documentation with subcommand structure

### Changed
- CLI now uses subcommands instead of flags: `hns summarize` instead of `hns --summarize`
- Updated README.md to document new CLI structure and subcommands

### Deprecated
- `--summarize` flag is now deprecated in favor of `hns summarize` subcommand (still works for backward compatibility)

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

