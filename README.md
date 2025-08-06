# MediaScrapeLang Engine (MSL Engine)

A Rust-based web scraping engine with a custom DSL (Domain Specific Language) for defining scraping pipelines.

## 🚀 Features

- **Custom DSL**: Write scraping scripts in a minimal, readable language
- **Link Traversal**: Follow links and extract data from multiple pages
- **Variable Extraction**: Extract text and attributes from HTML elements
- **Media Discovery**: Find and download images, videos, and audio files
- **Filtering**: Filter media by source URL patterns and file extensions
- **Async Processing**: Built with async Rust for efficient concurrent scraping
- **CLI Interface**: Easy-to-use command-line tool

## 📝 DSL Syntax

The MediaScrapeLang (MSL) DSL is designed to be minimal and readable:

```msl
open "https://example.com/users"

click ".user-card a"
  set user = text

  click ".post-list a"
    set post = attr("href").split("/")[-1]

    media
      image
        where src ~ "cdn.example.com"
        extensions jpg, png

      video
        where src ~ "cdn.example.com"
        extensions mp4, webm

    save to "./media/{user}/{post}"
```

### Commands

- `open "url"` - Navigate to a URL
- `click "selector"` - Click/follow links matching a CSS selector
- `set variable = value` - Extract and store a value
- `media` - Define media extraction blocks
- `save to "path"` - Save extracted media to a path

### Values

- `text` - Extract text content
- `attr("name")` - Extract attribute value
- `attr("name").split("/")[-1]` - Extract and process attribute

### Media Filters

- `where src ~ "pattern"` - Filter by source URL pattern
- `extensions jpg, png` - Filter by file extensions

## 🛠️ Installation

```bash
# Clone the repository
git clone <repository-url>
cd msl-engine

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## 📖 Usage

### Command Line Interface

```bash
# Run a script
msl run script.msl

# Parse and validate a script without executing
msl parse script.msl

# Enable verbose output
msl run script.msl --verbose
```

### Programmatic Usage

```rust
use msl_engine::{run_script, MslEngine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let script = r#"
        open "https://example.com"
        click ".user-card a"
          set user = text
        media
          image
            where src ~ "cdn.example.com"
            extensions jpg, png
          save to "./media/{user}"
    "#;
    
    run_script(script).await?;
    Ok(())
}
```

## 🏗️ Architecture

The MSL Engine is built with a modular architecture:

- **Parser** (`src/parser/`): Parses MSL scripts into structured AST
- **Scraper** (`src/scraper/`): Handles HTTP requests and HTML parsing
- **Engine** (`src/engine/`): Orchestrates the scraping process
- **CLI** (`src/cli/`): Command-line interface

### Key Components

- **MslScript**: Represents a parsed MSL script
- **MslEngine**: Main execution engine
- **Scraper**: HTTP client and HTML parser
- **MediaItem**: Represents discovered media files

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

## 📦 Dependencies

- **reqwest**: HTTP client
- **scraper**: HTML parsing
- **nom**: Parser combinator library
- **tokio**: Async runtime
- **clap**: CLI argument parsing
- **anyhow**: Error handling
- **tracing**: Logging

## 🚧 Development Status

- ✅ Parser implementation
- ✅ Basic scraper functionality
- ✅ Engine orchestration
- ✅ CLI interface
- 🔄 Variable templating in save paths
- 🔄 Advanced media filtering
- 🔄 Parallel processing
- 🔄 Headless browser support

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🎯 Roadmap

- [ ] Variable templating in save paths
- [ ] Advanced media filtering options
- [ ] Parallel processing for multiple pages
- [ ] Headless browser support (JavaScript rendering)
- [ ] Retry logic and error handling
- [ ] Rate limiting and polite scraping
- [ ] Export to different formats (JSON, CSV)
- [ ] Web interface for script editing 