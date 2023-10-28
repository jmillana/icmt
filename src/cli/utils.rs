use std::{env, io::Write, process::exit};

use question::{Answer, Question};
pub fn ask_for_confirmation(display: &str, default_answer: Option<Answer>) -> bool {
    let defaul_answer = default_answer.unwrap_or(Answer::YES);
    return Question::new(display)
        .yes_no()
        .until_acceptable()
        .default(defaul_answer)
        .ask()
        .expect("Couldn't ask question.")
        == Answer::YES;
}

pub fn get_current_shell() -> String {
    // Guess the current shell via env vars
    let mut shell = env::var("SHELL").unwrap_or_else(|_| String::new());
    if !shell.is_empty() {
        return shell;
    }

    shell = env::var("BASH").unwrap_or_else(|_| String::new());
    if !shell.is_empty() {
        return shell;
    }

    shell = env::var("ZSH_NAME").unwrap_or_else(|_| String::new());
    if !shell.is_empty() {
        return format!("/bin/{}", shell);
    }
    return shell;
}

pub fn write_to_history(shell: String, code: &str) {
    let history_file;
    let home_dir = std::env::var("HOME").unwrap();

    if shell.contains("bash") {
        history_file = home_dir + "/.bash_history";
    } else if shell.contains("zsh") {
        history_file = home_dir + "/.zsh_history";
    } else {
        return;
    }
    std::fs::OpenOptions::new()
        .append(true)
        .open(history_file)
        .map_or((), |mut file| {
            file.write_all(format!("{code}\n").as_bytes())
                .unwrap_or_else(|_| {
                    exit(1);
                });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_shell_from_shell_env() {
        env::set_var("SHELL", "/bin/zsh");
        let shell = get_current_shell();
        assert_eq!(shell, "/bin/zsh");
    }

    #[test]
    fn test_get_current_shell_from_bash_env() {
        env::remove_var("SHELL");
        env::set_var("BASH", "/bin/bash");
        let shell = get_current_shell();
        assert_eq!(shell, "/bin/bash");
    }

    #[test]
    fn test_get_current_shell_from_zsh_env() {
        env::remove_var("SHELL");
        env::remove_var("BASH");
        env::set_var("ZSH_NAME", "zsh");
        let shell = get_current_shell();
        assert_eq!(shell, "/bin/zsh");
    }
}
