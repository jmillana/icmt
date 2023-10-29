use crate::cli::utils::get_current_shell;
use colored::Colorize;
use std::{env, process::exit};

const MAX_TOKENS: usize = 1000;

pub struct Config {
    pub api_key: String,
    pub api_base: String,
    pub max_tokens: usize,
}

impl Config {
    pub fn new() -> Self {
        let shell = get_current_shell();
        if shell.is_empty() {
            println!(
                "{} {}",
                "Unable to guess the current shell".red(),
                "Supported: [bash, zsh]"
            );
        }
        let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
            println!("{}", "This program requires an OpenAI API key to run. Please set the OPENAI_API_KEY environment variable. https://github.com/m1guelpf/plz-cli#usage".red());
            exit(1);
        });
        let api_base = env::var("OPENAI_API_BASE")
            .unwrap_or_else(|_| String::from("https://api.openai.com/v1"));

        Self {
            api_key,
            api_base,
            max_tokens: MAX_TOKENS,
        }
    }
}
