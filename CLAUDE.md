# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a command-line dictionary tool called "SW" (Search Word) written in Rust. It queries the Free Dictionary API to provide word definitions, phonetics, examples, synonyms, and antonyms with colorful terminal output.

## Development Commands

### Building
```bash
# Debug build
cargo build

# Release build (for distribution)
cargo build --release

# Build for specific target (used in CI)
cargo build --release --target <target-triple>
```

### Testing
```bash
# Run all tests (excluding integration tests that require network)
cargo test

# Run a specific test
cargo test test_cli_with_word

# Run integration tests (require network access)
cargo test -- --ignored
```

### Running
```bash
# Basic word lookup
cargo run -- <word>

# Detailed lookup with synonyms/antonyms
cargo run -- <word> --detail

# Interactive mode
cargo run -- --interactive

# Help
cargo run -- --help
```

### Installation
```bash
# Install via Homebrew (if formula is tapped)
brew install szupzj18/tap/sw

# Install from source
cargo install --path .
```

## Code Architecture

### Single-File Structure
The entire application is contained in `src/main.rs` with the following organization:

1. **CLI Definition** (`Cli` struct) - Uses `clap` for argument parsing with three modes:
   - Single word lookup (`word` argument)
   - Detailed mode (`-d`/`--detail` flag)
   - Interactive mode (`-i`/`--interactive` flag)

2. **API Data Models** - Serde structs for Free Dictionary API responses:
   - `DictionaryResponse` - Main response container
   - `Meaning` - Word meanings with part of speech
   - `Definition` - Individual definitions with examples
   - `Phonetic` - Pronunciation information

3. **Core Functions**:
   - `lookup_word()` - Async HTTP request to API
   - `display_word_info()` - Colorful terminal formatting
   - `run_interactive_mode()` - REPL interface
   - `format_list()`/`print_non_empty_list()` - Helper utilities

4. **Comprehensive Tests** - Unit tests cover:
   - CLI argument parsing
   - JSON deserialization
   - Utility functions
   - Integration tests (marked `#[ignore]` for network access)

### Dependencies
- `clap` - Command-line argument parsing
- `reqwest` - HTTP client with async support
- `tokio` - Async runtime
- `serde`/`serde_json` - JSON serialization
- `colored` - Terminal color formatting

## Development Workflow

### Cursor Rules
The repository includes `.cursor/rules/development.mdc` which requires:
1. **Analysis first** - Understand requirements before coding
2. **Execution plan** - Provide detailed steps including files to modify, changes, impact, and risks
3. **Wait for confirmation** - Don't modify code without user approval
4. **Then execute** - Implement after plan is confirmed

### Release Process
1. Tag commits with `v*` pattern (e.g., `v0.1.0`)
2. GitHub Actions automatically builds binaries for:
   - Linux (x86_64)
   - macOS (x86_64 and aarch64)
3. Homebrew formula (`Formula/sw.rb`) is maintained separately

### Testing Strategy
- Unit tests are fast and don't require network
- Integration tests are marked with `#[ignore]` and require `-- --ignored` flag
- Tests cover CLI parsing, JSON deserialization, and utility functions
- API integration tests verify actual Free Dictionary API responses

## Key Implementation Details

### Error Handling
- Uses `Result<T, Box<dyn std::error::Error>>` for API errors
- User-friendly error messages with colored output
- Graceful handling of missing API data (optional fields)

### Async Pattern
- `#[tokio::main]` attribute on main function
- Async/await pattern for HTTP requests
- Proper error propagation with `?` operator

### Terminal Output
- Color-coded output using `colored` crate
- Different colors for: word, phonetic, part of speech, examples, synonyms, antonyms
- Clean formatting with proper indentation

### API Integration
- Uses Free Dictionary API (https://api.dictionaryapi.dev/)
- Handles array responses (multiple dictionary entries)
- Parses nested optional fields gracefully