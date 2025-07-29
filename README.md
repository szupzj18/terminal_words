# Terminal Words

A simple and elegant command-line dictionary tool written in Rust.

## Features

- **Fast word lookup** using the Free Dictionary API
- **Colorful terminal output** with clear formatting
- **Detailed information** including phonetics, definitions, examples, synonyms, and antonyms
- **Simple usage** with intuitive command-line interface

## Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd terminal_words
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/terminal_words`

## Usage

### Basic Usage

Look up a word:
```bash
cargo run -- <word>
# or after installation
terminal_words <word>
```

Example:
```bash
cargo run -- rust
```

### Detailed Information

Use the `-d` or `--detail` flag to get detailed information including synonyms and antonyms:
```bash
cargo run -- <word> --detail
# or
terminal_words <word> -d
```

Example:
```bash
cargo run -- rust --detail
```

## Examples

### Basic Lookup
```bash
$ cargo run -- rust
Looking up: rust

Word: rust
Phonetic: /rʌst/

Part of speech: noun
  1. a red or brown oxide coating on iron or steel caused by the action of oxygen and moisture
  2. a plant disease that produces a reddish-brown discoloration of leaves and stems; caused by various rust fungi
  3. the formation of reddish-brown ferric oxides on iron by low-temperature oxidation in the presence of water
```

### Detailed Lookup
```bash
$ cargo run -- rust --detail
Looking up: rust

Word: rust
Phonetic: /rʌst/

Part of speech: noun
  1. a red or brown oxide coating on iron or steel caused by the action of oxygen and moisture
     Example: The old car was covered in rust.
     Synonyms: corrosion, oxidation, tarnish
     Antonyms: 
  2. a plant disease that produces a reddish-brown discoloration of leaves and stems; caused by various rust fungi
     Example: The wheat crop was affected by rust.
     Synonyms: 
     Antonyms: 
```

## API Source

This tool uses the [Free Dictionary API](https://dictionaryapi.dev/) which provides free access to word definitions and related information.

## Dependencies

- `clap` - Command line argument parsing
- `reqwest` - HTTP client for API requests
- `tokio` - Async runtime
- `serde` - JSON serialization/deserialization
- `colored` - Terminal colors and formatting

## License

MIT License - feel free to use and modify as needed.