# hns - Hacker News Stories CLI

A command-line interface for fetching and displaying top stories from Hacker News.

## Description

`hns` is a simple, fast Rust command-line tool that pulls top stories from the official Hacker News API and displays them in a clean, readable format. The tool shows story titles, authors, timestamps, text content, and URLs in a structured way, making it easy to browse Hacker News without leaving your terminal.

## Features

- Fetch and display the top stories from Hacker News
- Configurable number of stories to display
- Clean formatting with proper handling of HTML content
- Special handling for "Show HN" posts
- Proper decoding of HTML entities

## Installation

Ensure you have Rust and Cargo installed. Then:

```bash
# Clone the repository
git clone https://github.com/yourusername/hns.git
cd hns

# Build and install
cargo install --path .
```

## Usage

Run the tool with default settings (shows 5 top stories):

```bash
hns
```

### Command-line Arguments

| Argument | Short | Description | Default | Range |
|----------|-------|-------------|---------|-------|
| `--max-stories` | `-m` | Maximum number of stories to display | 5 | 1-25 |
| `--help` | `-h` | Display help information | - | - |
| `--version` | `-V` | Display version information | - | - |

#### Examples

Display the top 10 stories:

```bash
hns -m 10
```

Display the maximum number of stories:

```bash
hns --max-stories 25
```

## Output Format

For each story, the tool displays:

1. A separator line
2. Timestamp, author, and story ID
3. Story title
4. For "Show HN" posts: URL followed by text content (if available)
5. For regular posts: Text content (if available) or URL

## Development

### Prerequisites

- Rust 1.70.0 or higher
- Cargo

### Development Tools

The project includes several files to help with development:

- `Makefile` - Common development tasks (run `make help` for details)
- `CHANGELOG.md` - Track changes between versions
- `CONTRIBUTING.md` - Guidelines for contributors

### Dependencies

- `clap` - Command-line argument parsing
- `reqwest` - HTTP client for API requests
- `tokio` - Asynchronous runtime
- `serde` - JSON serialization/deserialization
- `chrono` - Timestamp formatting
- `scraper` - HTML parsing
- `ego-tree` - Tree data structure for HTML parsing

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/hns.git
cd hns

# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Release Process

This project uses GitHub Actions for continuous integration and deployment. The workflow automatically:

1. Tests the codebase
2. Builds binaries for multiple platforms (Linux, macOS Intel/ARM, Windows)
3. Creates GitHub releases with packaged artifacts when tags are pushed
4. Publishes to crates.io when a new version is tagged

To create a new release:

```bash
# 1. Update the version in Cargo.toml
# 2. Commit the changes
git add Cargo.toml
git commit -m "Bump version to x.y.z"

# 3. Tag the commit
git tag -a vx.y.z -m "Release version x.y.z"

# 4. Push to GitHub with tags
git push && git push --tags
```

> **Note:** To enable publishing to crates.io, you must set the `CRATES_IO_TOKEN` secret in your GitHub repository settings.

## API

This tool uses the official Hacker News API:
- https://github.com/HackerNews/API

## License

This project is licensed under the BSD 3-Clause License - see the [LICENSE.md](LICENSE.md) file for details.
