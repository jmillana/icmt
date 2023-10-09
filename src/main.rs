#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use config::Config;
use question::{Answer, Question};
use spinners::{Spinner, Spinners};
use std::process::Command;

mod chat_gpt;
mod config;
mod git;
mod prompts;

use crate::chat_gpt::completions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Description of the command to execute
    prompt: Vec<String>,

    #[clap(short, long, default_value = "commit")]
    mode: String,

    // Gitmoji support
    #[clap(short = 'e', long)]
    gitmoji: bool,

    // Auto accept
    #[clap(short = 'y', long)]
    yes: bool,

    #[clap(short, long)]
    token_limit: Option<usize>,

    #[clap(short = 'H', long)]
    hint: Option<String>,
}

pub enum Mode {
    Commit,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Mode::Commit => write!(f, "commit"),
        }
    }
}

fn main() {
    println!(
        "{} {}",
        "🤖".bright_green(),
        "Welcome to commit AI!".bright_green()
    );
    let cli = Cli::parse();
    let config = Config::new();
    let chat_completions = completions::ChatCompletions::new(cli, config);

    match chat_completions.mode {
        Mode::Commit => {
            commit_workflow(chat_completions);
        }
    }
}

fn pprint(data: &String, lang: &str) {
    PrettyPrinter::new()
        .input_from_bytes(data.as_bytes())
        .language(lang)
        .grid(true)
        .print()
        .unwrap();
}

fn ask_for_confirmation(display: &str, default_answer: Option<Answer>) -> bool {
    let defaul_answer = default_answer.unwrap_or(Answer::YES);
    return Question::new(display)
        .yes_no()
        .until_acceptable()
        .default(defaul_answer)
        .ask()
        .expect("Couldn't ask question.")
        == Answer::YES;
}

fn commit_workflow(mut chat_completions: completions::ChatCompletions) {
    let mut spinner = Spinner::new(
        Spinners::BouncingBar,
        "Generating your commit message...".into(),
    );
    chat_completions.set_system_prompt(prompts::SystemPrompt::Commit);
    let auto_accept = chat_completions.cli.yes;

    let commit_changes = git::get_commit_changes().unwrap_or_else(|| {
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            "Failed to get commit changes.".red().to_string(),
        );
        std::process::exit(1);
    });

    let prompt = prompts::get_commit_user_prompt(commit_changes, &chat_completions.cli.hint);
    let mut commit_message = chat_completions.refine_loop(prompt, &mut spinner);

    if chat_completions.cli.gitmoji {
        commit_message = git::replace_gitmoji(commit_message);
    }

    pprint(&commit_message, "bash");

    if auto_accept || ask_for_confirmation(">> Apply the generated commit? [Y/n]", None) {
        let mut commit_cmd = "git commit -m '".to_string();
        commit_cmd.push_str(commit_message.as_str());
        commit_cmd.push_str("'");

        pprint(&commit_cmd, "bash");

        spinner = Spinner::new(Spinners::BouncingBar, "Executing...".into());
        let (stdout, _) = run_cmd(&commit_cmd, &"bash", &mut spinner);

        spinner.stop_and_persist(
            "✔".green().to_string().as_str(),
            "Command ran successfully".green().to_string(),
        );

        println!("{}", String::from_utf8_lossy(&stdout));
    }
}

fn run_cmd(command: &str, shell: &str, spinner: &mut Spinner) -> (Vec<u8>, Vec<u8>) {
    let output = Command::new(shell)
        .arg("-c")
        .arg(command)
        .output()
        .unwrap_or_else(|_| {
            spinner.stop_and_persist(
                "✖".red().to_string().as_str(),
                "Failed to execute the generated program.".red().to_string(),
            );
            std::process::exit(1);
        });

    if !output.status.success() {
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            "The program threw an error.".red().to_string(),
        );
        std::process::exit(1);
    }

    return (output.stdout, output.stderr);
}
