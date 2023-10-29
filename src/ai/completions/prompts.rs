use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub role: String,
    pub content: String,
}

impl Prompt {
    pub fn new() -> Self {
        return Prompt {
            role: String::new(),
            content: String::new(),
        };
    }

    pub fn build(role: String, content: String) -> Self {
        return Prompt { role, content };
    }
}

pub fn commit_system_prompt(gitmoji: bool) -> Prompt {
    /*
     *
     * */
    let mut content = String::new();
    content.push_str("You are an assistant to a programmer that will be generating commit messages for the code changes");
    content.push_str(
        "\nYour task if to identify the key changes and prepare a single commit message that encapsulates the changes accordingly.",
    );
    if gitmoji {
        content.push_str(" (using gitmoji emojis)");
    }

    content.push_str("\nFollowing the format: <type> ([optional scope]): <short description>\n\n[optional body]\n[optional footer]\n");
    return Prompt::build("system".to_string(), content);
}

pub fn squash_system_prompt(gitmoji: bool) -> Prompt {
    /*
     *
     * */
    let mut content = String::new();
    content.push_str(
        "You are an assistant to a programmer that will be generating squashed commit messages",
    );
    content.push_str(
        "\nYour task if to identify the key data present in the series of logs, and combine them into a single redacted commit message that drops meaningless information and simplifies the understanding of the implemented changes",
    );
    if gitmoji {
        content.push_str(" (using gitmoji emojis)");
    }

    content.push_str("\nFollowing the format: <type> ([optional scope]): <short description>\n\n[optional body]\n[optional footer]\n");
    return Prompt::build("system".to_string(), content);
}

pub fn get_commit_user_prompt(changes: Vec<String>, hint: &Option<String>) -> Prompt {
    let mut content = String::new();
    if let Some(hint) = hint {
        content.push_str(format!("Hint: {}", hint).as_str());
    }
    content.push_str("Provide a commit message for the following changes:\n");

    for change in changes {
        if change.as_str() == "" {
            continue;
        }
        content.push_str(change.as_str());
        content.push_str("\n");
    }
    return Prompt::build("user".to_string(), content);
}

pub fn get_squash_user_prompt(commits: Vec<String>, hint: &Option<String>) -> Prompt {
    let mut content = String::new();
    if let Some(hint) = hint {
        content.push_str(format!("Hint: {}", hint).as_str());
    }
    content.push_str(
        "Provide a concise squash of the providesd commits. Extract the key information and summaryse as much as you can the following commits :\n",
    );

    for commit in commits {
        content.push_str(commit.as_str());
        content.push_str("\n");
    }
    return Prompt::build("user".to_string(), content);
}
