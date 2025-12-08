use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "terminal_words")]
#[command(about = "A command-line dictionary tool", long_about = None)]
struct Cli {
    /// Word to look up
    word: Option<String>,
    
    /// Show detailed information (all definitions, examples, synonyms, antonyms)
    #[arg(short, long)]
    detail: bool,
    
    /// Interactive mode - continuous queries without exiting
    #[arg(short, long)]
    interactive: bool,
    
    /// Maximum number of definitions to show per part of speech (default: 3, use -d for all)
    #[arg(short = 'n', long, default_value = "3")]
    limit: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct DictionaryResponse {
    word: String,
    phonetic: Option<String>,
    phonetics: Option<Vec<Phonetic>>,
    meanings: Vec<Meaning>,
    license: Option<License>,
    source_urls: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Phonetic {
    text: Option<String>,
    audio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Meaning {
    part_of_speech: Option<String>,
    definitions: Vec<Definition>,
    synonyms: Option<Vec<String>>,
    antonyms: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Definition {
    definition: String,
    example: Option<String>,
    synonyms: Option<Vec<String>>,
    antonyms: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct License {
    name: String,
    url: String,
}

async fn lookup_word(word: &str) -> Result<Vec<DictionaryResponse>, Box<dyn std::error::Error>> {
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}"
, word);
    
    let response = reqwest::get(&url).await?;
    
    if response.status().is_success() {
        let definitions: Vec<DictionaryResponse> = response.json().await?;
        Ok(definitions)
    } else {
        Err("Word not found or API error".into())
    }
}

/// Format a non-empty list as "label item1, item2, ..." or None if empty/missing
fn format_list(items: &Option<Vec<String>>) -> Option<String> {
    items
        .as_ref()
        .filter(|v| !v.is_empty())
        .map(|v| v.join(", "))
}

/// Print a non-empty list with the given indent and label
fn print_non_empty_list(indent: &str, label: ColoredString, items: &Option<Vec<String>>) {
    if let Some(text) = format_list(items) {
        println!("{}{} {}", indent, label, text);
    }
}

/// Display options for controlling output verbosity
struct DisplayOptions {
    /// Show all content (definitions, examples, synonyms, antonyms)
    detailed: bool,
    /// Maximum definitions per part of speech (ignored if detailed is true)
    limit: usize,
}

fn display_word_info(response: &DictionaryResponse, options: &DisplayOptions) {
    println!("\n{} {}", "Word:".bright_green().bold(), response.word.bright_white().bold());
    
    if let Some(phonetic) = &response.phonetic {
        println!("{} {}", "Phonetic:".bright_blue(), phonetic.bright_yellow());
    }
    
    // Use iterator chain to simplify nested Option traversal
    for text in response.phonetics.iter().flatten().filter_map(|p| p.text.as_ref()) {
        println!("{} {}", "Pronunciation:".bright_blue(), text.bright_yellow());
    }
    
    println!();
    
    for meaning in &response.meanings {
        // Use unwrap_or to simplify if-else
        let pos = meaning.part_of_speech.as_deref().unwrap_or("unknown");
        println!("{} {}", "Part of speech:".bright_magenta().bold(), pos.bright_cyan());
        
        // Determine how many definitions to show
        let total_defs = meaning.definitions.len();
        let show_count = if options.detailed { total_defs } else { total_defs.min(options.limit) };
        
        for (i, def) in meaning.definitions.iter().take(show_count).enumerate() {
            println!("  {} {}", format!("{}.", i + 1).bright_green(), def.definition.white());
            
            // In detailed mode: show all examples
            // In normal mode: only show example for the first definition
            if options.detailed || i == 0 {
                if let Some(example) = &def.example {
                    println!("     {} {}", "Example:".bright_blue(), example.italic());
                }
            }
            
            if options.detailed {
                print_non_empty_list("     ", "Synonyms:".bright_yellow(), &def.synonyms);
                print_non_empty_list("     ", "Antonyms:".bright_red(), &def.antonyms);
            }
        }
        
        // Show hint if there are more definitions
        if !options.detailed && total_defs > show_count {
            println!("     {} (+{} more, use -d for all)", "...".dimmed(), total_defs - show_count);
        }
        
        if options.detailed {
            print_non_empty_list("  ", "Synonyms:".bright_yellow(), &meaning.synonyms);
            print_non_empty_list("  ", "Antonyms:".bright_red(), &meaning.antonyms);
        }
        
        println!();
    }
    
    // Use and_then to simplify nested Option access
    if options.detailed {
        if let Some(url) = response.source_urls.as_ref().and_then(|urls| urls.first()) {
            println!("{} {}", "Source:".bright_blue(), url.underline());
        }
    }
}

async fn lookup_and_display(word: &str, options: &DisplayOptions) {
    println!("{} {}", "Looking up:".bright_green(), word.bright_white().bold());
    
    match lookup_word(word).await {
        Ok(definitions) => {
            for definition in definitions {
                display_word_info(&definition, options);
            }
        }
        Err(e) => {
            println!("{} {}", "Error:".bright_red().bold(), e.to_string().bright_red());
        }
    }
}

async fn run_interactive_mode(options: &DisplayOptions) {
    println!("{}", "🔄 Interactive Mode".bright_cyan().bold());
    println!("{}", "Type a word to look up, or 'q'/'quit'/'exit' to exit.".bright_blue());
    println!();
    
    loop {
        print!("{} ", "sw>".bright_green().bold());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                // EOF reached (Ctrl+D)
                println!();
                println!("{}", "Goodbye! 👋".bright_cyan());
                break;
            }
            Ok(_) => {
                let word = input.trim();
                
                if word.is_empty() {
                    continue;
                }
                
                if is_exit_command(word) {
                    println!("{}", "Goodbye! 👋".bright_cyan());
                    break;
                }
                
                lookup_and_display(word, options).await;
            }
            Err(e) => {
                println!("{} {}", "Error reading input:".bright_red(), e);
                break;
            }
        }
    }
}

/// Check if the input is an exit command
fn is_exit_command(input: &str) -> bool {
    matches!(input, "q" | "quit" | "exit")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    let options = DisplayOptions {
        detailed: cli.detail,
        limit: cli.limit,
    };
    
    if cli.interactive {
        // Interactive mode
        run_interactive_mode(&options).await;
    } else if let Some(word) = cli.word {
        // Single word lookup mode
        lookup_and_display(&word, &options).await;
    } else {
        // No word provided and not in interactive mode
        println!("{}", "Error: Please provide a word to look up, or use -i for interactive mode.".bright_red());
        println!("{}", "Usage: sw <word> or sw -i".bright_yellow());
        std::process::exit(1);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    // ==================== CLI Tests ====================

    #[test]
    fn test_cli_with_word() {
        let cli = Cli::try_parse_from(["sw", "hello"]).unwrap();
        assert_eq!(cli.word, Some("hello".to_string()));
        assert!(!cli.detail);
        assert!(!cli.interactive);
        assert_eq!(cli.limit, 3); // default limit
    }

    #[test]
    fn test_cli_with_detail_flag() {
        let cli = Cli::try_parse_from(["sw", "hello", "-d"]).unwrap();
        assert_eq!(cli.word, Some("hello".to_string()));
        assert!(cli.detail);
        assert!(!cli.interactive);
    }

    #[test]
    fn test_cli_with_interactive_flag() {
        let cli = Cli::try_parse_from(["sw", "-i"]).unwrap();
        assert_eq!(cli.word, None);
        assert!(!cli.detail);
        assert!(cli.interactive);
    }

    #[test]
    fn test_cli_with_interactive_and_detail_flags() {
        let cli = Cli::try_parse_from(["sw", "-i", "-d"]).unwrap();
        assert_eq!(cli.word, None);
        assert!(cli.detail);
        assert!(cli.interactive);
    }

    #[test]
    fn test_cli_no_args() {
        let cli = Cli::try_parse_from(["sw"]).unwrap();
        assert_eq!(cli.word, None);
        assert!(!cli.detail);
        assert!(!cli.interactive);
        assert_eq!(cli.limit, 3); // default limit
    }

    #[test]
    fn test_cli_with_limit_flag() {
        let cli = Cli::try_parse_from(["sw", "hello", "-n", "5"]).unwrap();
        assert_eq!(cli.word, Some("hello".to_string()));
        assert_eq!(cli.limit, 5);
    }

    #[test]
    fn test_cli_with_limit_long_flag() {
        let cli = Cli::try_parse_from(["sw", "hello", "--limit", "10"]).unwrap();
        assert_eq!(cli.word, Some("hello".to_string()));
        assert_eq!(cli.limit, 10);
    }

    #[test]
    fn test_cli_with_limit_and_detail() {
        let cli = Cli::try_parse_from(["sw", "hello", "-n", "2", "-d"]).unwrap();
        assert_eq!(cli.word, Some("hello".to_string()));
        assert_eq!(cli.limit, 2);
        assert!(cli.detail); // detail mode ignores limit
    }

    // ==================== Exit Command Tests ====================

    #[test]
    fn test_exit_commands() {
        assert!(is_exit_command("q"));
        assert!(is_exit_command("quit"));
        assert!(is_exit_command("exit"));
    }

    #[test]
    fn test_non_exit_commands() {
        assert!(!is_exit_command("hello"));
        assert!(!is_exit_command(""));
        assert!(!is_exit_command("quit "));
        assert!(!is_exit_command("Q")); // Case sensitive
    }

    // ==================== Format List Tests ====================

    #[test]
    fn test_format_list_with_items() {
        let items = Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(format_list(&items), Some("a, b, c".to_string()));
    }

    #[test]
    fn test_format_list_with_single_item() {
        let items = Some(vec!["only".to_string()]);
        assert_eq!(format_list(&items), Some("only".to_string()));
    }

    #[test]
    fn test_format_list_with_empty_vec() {
        let items: Option<Vec<String>> = Some(vec![]);
        assert_eq!(format_list(&items), None);
    }

    #[test]
    fn test_format_list_with_none() {
        let items: Option<Vec<String>> = None;
        assert_eq!(format_list(&items), None);
    }

    // ==================== JSON Deserialization Tests ====================

    #[test]
    fn test_deserialize_dictionary_response() {
        let json = r#"
        {
            "word": "hello",
            "phonetic": "/həˈloʊ/",
            "phonetics": [
                {"text": "/həˈloʊ/", "audio": "https://example.com/audio.mp3"}
            ],
            "meanings": [
                {
                    "partOfSpeech": "noun",
                    "definitions": [
                        {
                            "definition": "A greeting",
                            "example": "Hello, world!"
                        }
                    ],
                    "synonyms": ["hi", "hey"],
                    "antonyms": ["goodbye"]
                }
            ],
            "license": {
                "name": "CC BY-SA 3.0",
                "url": "https://creativecommons.org/licenses/by-sa/3.0"
            },
            "sourceUrls": ["https://en.wiktionary.org/wiki/hello"]
        }
        "#;

        let response: DictionaryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.word, "hello");
        assert_eq!(response.phonetic, Some("/həˈloʊ/".to_string()));
        assert_eq!(response.meanings.len(), 1);
        assert_eq!(response.meanings[0].part_of_speech, Some("noun".to_string()));
        assert_eq!(response.meanings[0].definitions.len(), 1);
        assert_eq!(response.meanings[0].definitions[0].definition, "A greeting");
        assert_eq!(response.meanings[0].definitions[0].example, Some("Hello, world!".to_string()));
    }

    #[test]
    fn test_deserialize_minimal_response() {
        let json = r#"
        {
            "word": "test",
            "meanings": [
                {
                    "definitions": [
                        {"definition": "A simple test"}
                    ]
                }
            ]
        }
        "#;

        let response: DictionaryResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.word, "test");
        assert!(response.phonetic.is_none());
        assert!(response.phonetics.is_none());
        assert_eq!(response.meanings.len(), 1);
    }

    #[test]
    fn test_deserialize_response_array() {
        let json = r#"
        [
            {
                "word": "hello",
                "meanings": [
                    {
                        "partOfSpeech": "exclamation",
                        "definitions": [{"definition": "Used as a greeting"}]
                    }
                ]
            }
        ]
        "#;

        let responses: Vec<DictionaryResponse> = serde_json::from_str(json).unwrap();
        assert_eq!(responses.len(), 1);
        assert_eq!(responses[0].word, "hello");
    }

    // ==================== Integration Tests ====================
    // These tests require network access, run with: cargo test -- --ignored

    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_lookup_word_success() {
        let result = lookup_word("hello").await;
        assert!(result.is_ok());
        let definitions = result.unwrap();
        assert!(!definitions.is_empty());
        assert_eq!(definitions[0].word, "hello");
    }

    #[tokio::test]
    #[ignore = "requires network access"]
    async fn test_lookup_word_not_found() {
        let result = lookup_word("asdfghjklqwerty123456").await;
        assert!(result.is_err());
    }
}
