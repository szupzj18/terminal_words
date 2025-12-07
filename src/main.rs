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
    
    /// Show detailed information
    #[arg(short, long)]
    detail: bool,
    
    /// Interactive mode - continuous queries without exiting
    #[arg(short, long)]
    interactive: bool,
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

fn display_word_info(response: &DictionaryResponse, detailed: bool) {
    println!("\n{} {}", "Word:".bright_green().bold(), response.word.bright_white().bold());
    
    if let Some(phonetic) = &response.phonetic {
        println!("{} {}", "Phonetic:".bright_blue(), phonetic.bright_yellow());
    }
    
    if let Some(phonetics) = &response.phonetics {
        for phonetic in phonetics {
            if let Some(text) = &phonetic.text {
                println!("{} {}", "Pronunciation:".bright_blue(), text.bright_yellow());
            }
        }
    }
    
    println!();
    
    for meaning in &response.meanings {
        if let Some(pos) = &meaning.part_of_speech {
            println!("{} {}", "Part of speech:".bright_magenta().bold(), pos.bright_cyan());
        } else {
            println!("{} {}", "Part of speech:".bright_magenta().bold(), "unknown".bright_cyan());
        }
        
        for (i, definition) in meaning.definitions.iter().enumerate() {
            println!("  {} {}", format!("{}.", i + 1).bright_green(), definition.definition.white());
            
            if let Some(example) = &definition.example {
                println!("     {} {}", "Example:".bright_blue(), example.italic());
            }
            
            if detailed {
                if let Some(synonyms) = &definition.synonyms {
                    if !synonyms.is_empty() {
                        println!("     {} {}", "Synonyms:".bright_yellow(), synonyms.join(", "));
                    }
                }
                
                if let Some(antonyms) = &definition.antonyms {
                    if !antonyms.is_empty() {
                        println!("     {} {}", "Antonyms:".bright_red(), antonyms.join(", "));
                    }
                }
            }
        }
        
        if detailed {
            if let Some(synonyms) = &meaning.synonyms {
                if !synonyms.is_empty() {
                    println!("  {} {}", "Synonyms:".bright_yellow(), synonyms.join(", "));
                }
            }
            
            if let Some(antonyms) = &meaning.antonyms {
                if !antonyms.is_empty() {
                    println!("  {} {}", "Antonyms:".bright_red(), antonyms.join(", "));
                }
            }
        }
        
        println!();
    }
    
    if detailed {
        if let Some(source_urls) = &response.source_urls {
            println!("{} {}", "Source:".bright_blue(), source_urls[0].underline());
        }
    }
}

async fn lookup_and_display(word: &str, detailed: bool) {
    println!("{} {}", "Looking up:".bright_green(), word.bright_white().bold());
    
    match lookup_word(word).await {
        Ok(definitions) => {
            for definition in definitions {
                display_word_info(&definition, detailed);
            }
        }
        Err(e) => {
            println!("{} {}", "Error:".bright_red().bold(), e.to_string().bright_red());
        }
    }
}

async fn run_interactive_mode(detailed: bool) {
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
                
                lookup_and_display(word, detailed).await;
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
    
    if cli.interactive {
        // Interactive mode
        run_interactive_mode(cli.detail).await;
    } else if let Some(word) = cli.word {
        // Single word lookup mode
        lookup_and_display(&word, cli.detail).await;
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
