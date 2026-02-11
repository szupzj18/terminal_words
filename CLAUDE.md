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
# Basic word lookup (shows up to 3 definitions per part of speech by default)
cargo run -- <word>

# Limit definitions per part of speech
cargo run -- <word> -n 5

# Detailed lookup with all definitions, synonyms/antonyms
cargo run -- <word> --detail

# Interactive mode
cargo run -- --interactive

# Interactive mode with detail
cargo run -- -i -d

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

1. **CLI Definition** (`Cli` struct) - Uses `clap` for argument parsing with four options:
   - Single word lookup (`word` argument)
   - Detailed mode (`-d`/`--detail` flag) - show all definitions, examples, synonyms, antonyms
   - Interactive mode (`-i`/`--interactive` flag) - REPL-style continuous queries
   - Limit (`-n`/`--limit`, default: 3, min: 1) - max definitions per part of speech (ignored in detail mode)

2. **API Data Models** - Serde structs for Free Dictionary API responses:
   - `DictionaryResponse` - Main response container (includes `license` and `source_urls` fields)
   - `Meaning` - Word meanings with part of speech (uses `#[serde(rename_all = "camelCase")]`)
   - `Definition` - Individual definitions with examples, synonyms, antonyms
   - `Phonetic` - Pronunciation information (text and audio)
   - `License` - License metadata (name and url)

3. **Core Functions**:
   - `lookup_word()` - Async HTTP request to API (with URL encoding via `urlencoding`)
   - `display_word_info()` - Colorful terminal formatting, controlled by `DisplayOptions`
   - `lookup_and_display()` - Combines lookup and display with error handling
   - `run_interactive_mode()` - REPL interface with exit command support
   - `is_exit_command()` - Checks for quit commands (`q`/`quit`/`exit`)
   - `format_list()`/`print_non_empty_list()` - Helper utilities

4. **Display Control** (`DisplayOptions` struct):
   - `detailed: bool` - Whether to show all content
   - `limit: u64` - Max definitions per part of speech (ignored when `detailed` is true)

5. **Comprehensive Tests** - Unit tests cover:
   - CLI argument parsing (including `--limit`, `--detail`, `--interactive` combinations)
   - Exit command validation
   - Format list utilities
   - JSON deserialization (full and minimal responses)
   - Integration tests (marked `#[ignore]` for network access)

### Dependencies

- `clap` - Command-line argument parsing (with `derive` feature)
- `reqwest` - HTTP client with async support (with `json` and `blocking` features)
- `tokio` - Async runtime (with `full` feature)
- `serde`/`serde_json` - JSON serialization (with `derive` feature)
- `colored` - Terminal color formatting
- `urlencoding` - URL encoding for word queries

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
- Tests cover CLI parsing (word, detail, interactive, limit, edge cases like limit=0), exit command validation, format list utilities, and JSON deserialization
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
- URL encodes word queries via `urlencoding` crate
- Handles array responses (multiple dictionary entries)
- Parses nested optional fields gracefully
