use crate::cli;
use colored::Colorize;
use spinners::{Spinner, Spinners};
use std::process::Command;

pub struct RunArgs {
    pub command: String,
    pub shell: Option<String>,
    pub spinner_flag: bool,
}

impl RunArgs {
    pub fn build(command: String) -> Self {
        return Self {
            command,
            shell: None,
            spinner_flag: false,
        };
    }
}
pub struct Output {
    pub stdout: String,
    pub stderr: String,
}

impl Output {
    pub fn from_u8_vec(stdout: Vec<u8>, stderr: Vec<u8>) -> Self {
        return Self {
            stdout: String::from_utf8_lossy(&stdout).to_string(),
            stderr: String::from_utf8_lossy(&stderr).to_string(),
        };
    }
}

pub fn run_with_spinner(command: String, shell: Option<String>) -> Result<Output, String> {
    let mut spinner = Spinner::new(Spinners::BouncingBar, "Executing...".into());
    let mut new_args = RunArgs::build(command.clone());
    new_args.shell = shell;
    // Make sure spinner flag is false, otherwise this will cause an infinite
    // recursion loop
    new_args.spinner_flag = false;
    let output = run(new_args);
    if output.is_ok() {
        spinner.stop_and_persist(
            "✔".green().to_string().as_str(),
            "Command ran successfully".green().to_string(),
        );
    } else {
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            format!("Failed to run the command: {}", command)
                .red()
                .to_string(),
        );
    }
    return output;
}

pub fn run(args: RunArgs) -> Result<Output, String> {
    if args.spinner_flag {
        return run_with_spinner(args.command, args.shell);
    }

    let shell = match args.shell {
        Some(shell) => {
            if shell.is_empty() {
                cli::utils::get_current_shell()
            } else {
                shell
            }
        }
        _ => cli::utils::get_current_shell(),
    };
    if shell.is_empty() {
        return Err("Unable to guess the current shell".to_string());
    }
    let command = args.command;
    let output = Command::new(shell.clone()).arg("-c").arg(&command).output();
    if output.is_err() {
        return Err(format!(
            "Failed to execute the generated program.\n{}",
            command
        ));
    }
    let output = output.unwrap();

    let command_output = Output::from_u8_vec(output.stdout, output.stderr);

    if !output.status.success() {
        return Err(format!(
            "The command {} threw an error.\n {}",
            command, command_output.stderr
        ));
    }
    cli::utils::write_to_history(shell, &command.as_str());
    return Ok(command_output);
}
