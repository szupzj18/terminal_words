use clap::Parser;
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "terminal_words")]
#[command(about = "A command-line dictionary tool", long_about = None)]
struct Cli {
    /// Word to look up
    word: String,
    
    /// Show detailed information
    #[arg(short, long)]
    detail: bool,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    println!("{} {}", "Looking up:".bright_green(), cli.word.bright_white().bold());
    
    match lookup_word(&cli.word).await {
        Ok(definitions) => {
            for definition in definitions {
                display_word_info(&definition, cli.detail);
            }
        }
        Err(e) => {
            println!("{} {}", "Error:".bright_red().bold(), e.to_string().bright_red());
            std::process::exit(1);
        }
    }
    
    Ok(())
}
