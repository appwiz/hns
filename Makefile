# Makefile for hns

# Variables
BINARY_NAME = hns
VERSION = $(shell grep -m1 'version = ' Cargo.toml | cut -d '"' -f2)

# Default target
.PHONY: all
all: build

# Build binary
.PHONY: build
build:
	cargo build

# Run binary
.PHONY: run
run:
	cargo run

# Run with specific arguments
.PHONY: run-max
run-max:
	cargo run -- --max-stories 10

# Build optimized binary
.PHONY: release
release:
	cargo build --release

# Run tests
.PHONY: test
test:
	cargo test

# Run tests with coverage
.PHONY: coverage
coverage:
	cargo tarpaulin

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# Format code
.PHONY: fmt
fmt:
	cargo fmt

# Check formatting
.PHONY: check-fmt
check-fmt:
	cargo fmt -- --check

# Lint code
.PHONY: lint
lint:
	cargo clippy -- -D warnings

# Install locally
.PHONY: install
install: release
	cp target/release/$(BINARY_NAME) $(HOME)/.local/bin/

# Prepare for release
.PHONY: prepare-release
prepare-release:
	@echo "Current version: $(VERSION)"
	@echo "Preparing to release version $(VERSION)"
	@echo "1. Updating CHANGELOG.md"
	@echo "2. Run 'git add CHANGELOG.md'"
	@echo "3. Run 'git commit -m \"Prepare release v$(VERSION)\"'"
	@echo "4. Run 'git tag -a v$(VERSION) -m \"Release version $(VERSION)\"'"
	@echo "5. Run 'git push && git push --tags'"

# Add a new entry to CHANGELOG.md
.PHONY: changelog-entry
changelog-entry:
	@echo "Adding a new entry to CHANGELOG.md for version $(VERSION)..."
	@DATE=$$(date +"%Y-%m-%d"); \
	TMP_CHANGELOG="/tmp/hns-changelog-$$$$.md"; \
	if grep -q "## \[$(VERSION)\]" CHANGELOG.md; then \
		sed "s/## \[$(VERSION)\] - [0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]/## [$(VERSION)] - $$DATE/" CHANGELOG.md > $$TMP_CHANGELOG; \
		mv $$TMP_CHANGELOG CHANGELOG.md; \
		echo "Updated existing entry for version $(VERSION) with today's date"; \
	else \
		awk -v version="$(VERSION)" -v date="$$DATE" ' \
			/## \[Unreleased\]/ { \
				print $$0; \
				getline; \
				print $$0; \
				print "\n## [" version "] - " date; \
				print "### Added"; \
				print "- "; \
				print ""; \
				next; \
			} \
			{print} \
		' CHANGELOG.md > $$TMP_CHANGELOG; \
		mv $$TMP_CHANGELOG CHANGELOG.md; \
		echo "Created new entry for version $(VERSION)"; \
		echo "Please edit CHANGELOG.md to add release notes"; \
	fi
	@echo "Changelog updated. Don't forget to review and commit the changes!"

# Tag version from Cargo.toml (assumes CHANGELOG.md is already updated)
.PHONY: tag-version
tag-version:
	@echo "Tagging version $(VERSION) from Cargo.toml"
	@echo "Assuming CHANGELOG.md has already been updated for version $(VERSION)"
	@git add Cargo.toml CHANGELOG.md
	@git commit -m "Bump version to $(VERSION)"
	@git tag -a v$(VERSION) -m "Release version $(VERSION)"
	@echo "Created tag v$(VERSION). To push, run: git push && git push --tags"

# Help
.PHONY: help
help:
	@echo "Makefile for $(BINARY_NAME) v$(VERSION)"
	@echo ""
	@echo "Targets:"
	@echo "  all         Build the binary (default)"
	@echo "  build       Build the binary"
	@echo "  run         Run the binary"
	@echo "  run-max     Run with 10 stories"
	@echo "  release     Build optimized binary"
	@echo "  test        Run tests"
	@echo "  coverage    Run tests with coverage"
	@echo "  clean       Clean build artifacts"
	@echo "  fmt         Format code"
	@echo "  check-fmt   Check formatting"
	@echo "  lint        Lint code"
	@echo "  install     Install binary to ~/.local/bin"
	@echo "  prepare-release  Steps to prepare a release"
	@echo "  changelog-entry  Add a new entry to CHANGELOG.md for the current version"
	@echo "  tag-version      Tag the current version (assumes Cargo.toml and CHANGELOG.md are updated)"
	@echo "  help        Show this help"
