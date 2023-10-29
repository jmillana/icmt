#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use config::Config;
use env_logger;

mod ai;
mod cli;
mod commands;
mod config;
mod git;

use crate::ai::completions::{chatgpty, prompts};

fn main() {
    println!(
        "{} {}",
        "🤖".bright_green(),
        "Welcome to commit AI!".bright_green()
    );
    env_logger::init();
    let cli = cli::Cli::parse();
    let mut config = Config::new();
    match cli.token_limit {
        Some(limit) => config.max_tokens = limit,
        _ => (),
    };
    let chat_completions = chatgpty::GptyCompletions::new(config);
    match &cli.command {
        cli::Commands::Commit(args) => {
            commands::commit::commit_workflow(chat_completions, &args);
        }
        cli::Commands::Squash(args) => {
            commands::squash::squash_workflow(chat_completions, &args);
        }
    };
}

fn pprint(data: &String, lang: &str) {
    PrettyPrinter::new()
        .input_from_bytes(data.as_bytes())
        .language(lang)
        .grid(true)
        .print()
        .unwrap();
}
