# SW (Search Word)

A simple and elegant command-line dictionary tool written in Rust.

## Features

- **Fast word lookup** using the Free Dictionary API
- **Colorful terminal output** with clear formatting
- **Detailed information** including phonetics, definitions, examples, synonyms, and antonyms
- **Interactive mode** for continuous word lookups without restarting
- **Configurable output** - limit the number of definitions shown per part of speech
- **Simple usage** with intuitive command-line interface

## Installation

### Via Homebrew (Recommended)

```bash
brew tap szupzj18/tap
brew install sw
```

### From Source

1. Clone this repository:

```bash
git clone https://github.com/szupzj18/sw.git
cd sw
```

2. Build the project:

```bash
cargo build --release
```

3. The binary will be available at `target/release/sw`

## Usage

### Basic Usage

Look up a word:

```bash
sw <word>
```

Example:

```bash
sw rust
```

### Detailed Information

Use the `-d` or `--detail` flag to get detailed information including all definitions, examples, synonyms and antonyms:

```bash
sw <word> --detail
# or
sw <word> -d
```

### Limit Definitions

By default, up to 3 definitions are shown per part of speech. Use `-n` or `--limit` to change this:

```bash
# Show up to 5 definitions per part of speech
sw <word> -n 5

# Show only 1 definition per part of speech
sw <word> --limit 1
```

### Interactive Mode

Use `-i` or `--interactive` for a REPL-style interface to look up multiple words without restarting:

```bash
sw -i
# or combine with detail mode
sw -i -d
```

Type `q`, `quit`, or `exit` to leave interactive mode.

## Examples

### Basic Lookup (default: up to 3 definitions)

```bash
$ sw hello
Looking up: hello

Word: hello
Phonetic: /həˈloʊ/

Part of speech: noun
  1. "Hello!" or an equivalent greeting.
  2. ...  (+1 more, use -d for all)

Part of speech: verb
  1. To greet with "hello".
```

### Detailed Lookup

```bash
$ sw hello --detail
Looking up: hello

Word: hello
Phonetic: /həˈloʊ/

Part of speech: noun
  1. "Hello!" or an equivalent greeting.
  2. A greeting (salutation) said when meeting someone or acknowledging someone's arrival or presence.
     Example: "Hello, everyone."
  3. A call for response if it is not clear if anyone is present or listening...
     Example: "Hello? Is anyone there?"
  Synonyms: greeting
```

### Interactive Mode

```bash
$ sw -i
🔄 Interactive Mode
Type a word to look up, or 'q'/'quit'/'exit' to exit.

sw> hello
Looking up: hello
...

sw> quit
Goodbye! 👋
```

## API Source

This tool uses the [Free Dictionary API](https://dictionaryapi.dev/) which provides free access to word definitions and related information.

## Dependencies

- `clap` - Command line argument parsing
- `reqwest` - HTTP client for API requests
- `tokio` - Async runtime
- `serde` / `serde_json` - JSON serialization/deserialization
- `colored` - Terminal colors and formatting
- `urlencoding` - URL encoding for word queries

## License

MIT License - feel free to use and modify as needed.
