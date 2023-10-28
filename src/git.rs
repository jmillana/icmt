use log;
use regex::Regex;
use std::process::Command;

#[derive(Debug)]
pub struct CommitHistory {
    pub commits: Vec<CommitMsg>,
}

#[derive(Debug)]
pub struct CommitMsg {
    pub hash: String,
    pub author: String,
    pub date: String,
    pub title: String,
    pub body: Option<String>,
}

impl CommitMsg {
    pub fn from_str(commit: &str) -> Self {
        let lines = commit.split("\n").collect::<Vec<&str>>();

        let body;
        if lines.len() < 5 {
            body = None;
        } else {
            body = Some(lines[5..].concat().to_string());
        }
        return Self {
            hash: lines[0].split("commit").last().unwrap().trim().to_string(),
            author: lines[1].split("Author:").last().unwrap().trim().to_string(),
            date: lines[2].split("Date:").last().unwrap().trim().to_string(),
            title: lines[4].trim().to_string(),
            body,
        };
    }
    pub fn to_string(self: &Self) -> String {
        let body = match &self.body {
            Some(body) => body,
            None => "",
        };
        let text = format!(
            "Commit: {}\nAuthor: {}\nDate: {}\n{}\n\n{}",
            self.hash, self.author, self.date, self.title, body,
        );
        return text;
    }
}

fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, _) in text.match_indices(r) {
        if last != index {
            result.push(&text[last..index]);
            last = index;
        }
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

impl CommitHistory {
    pub fn from_string(data: &Vec<String>) -> Self {
        let mut commits = Vec::new();
        let re = Regex::new(r"commit [a-z0-9]{40}.*").expect("Invalid regex");

        let single_string = data.join("\n");
        let parts = split_keep(&re, &single_string);
        for part in parts {
            commits.push(CommitMsg::from_str(part));
        }
        return Self { commits };
    }

    pub fn to_string(self: &Self) -> String {
        let mut text = String::new();
        for commit in self.commits.as_slice() {
            text += commit.to_string().as_str();
            text += "\n";
        }
        return text;
    }
}

pub fn get_commit_changes() -> Option<Vec<String>> {
    // Get the changes in the working directory
    let diff = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()
        .unwrap_or_else(|_| {
            println!("Failed to execute git diff.");
            std::process::exit(1);
        });

    let diff = String::from_utf8_lossy(&diff.stdout);
    if diff.is_empty() {
        return None;
    }
    // Skip first line
    let diff = diff
        .lines()
        .skip(1)
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    return Some(diff);
}

pub fn get_branch_name() -> Option<String> {
    let branch_name_cmd = Command::new("git")
        .arg("name-rev")
        .arg("HEAD")
        .output()
        .unwrap_or_else(|_| {
            println!("Failed to execute git name-rev.");
            std::process::exit(1);
        });

    let branch_name = String::from_utf8_lossy(&branch_name_cmd.stdout);
    if branch_name.is_empty() {
        return None;
    }
    return Some(branch_name.split_whitespace().nth(1).unwrap().to_string());
}

pub fn restore_head(n_commits: usize) {
    Command::new("git")
        .arg("reset")
        .arg(format!("HEAD~{}", n_commits))
        .output()
        .unwrap_or_else(|_| {
            println!("Failed to execute git log.");
            std::process::exit(1);
        });
}

pub fn get_commits(base_branch: &str) -> CommitHistory {
    // Get all the commits of the current branch
    let commits_cmd = Command::new("git")
        .arg("log")
        .arg("HEAD")
        .arg("--not")
        .arg(base_branch)
        .output()
        .unwrap_or_else(|_| {
            println!("Failed to execute git log.");
            std::process::exit(1);
        });
    let commits = String::from_utf8_lossy(&commits_cmd.stdout);

    let commits = commits
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let history = CommitHistory::from_string(&commits);
    return history;
}

fn get_gitmojis(tag: String) -> String {
    let awk_cmd = "awk '{print $1 $3}'";
    let gitmoji = Command::new("bash")
        .arg("-c")
        .arg(format!("gitmoji -s {} | {}", tag, awk_cmd))
        .output()
        .unwrap_or_else(|_| {
            println!("Failed to execute gitmoji.");
            std::process::exit(1);
        });
    // Check if gitmoji is empty
    if gitmoji.stdout.is_empty() {
        log::debug!("No gitmojis found for tag {}", tag);
        return tag;
    }

    let emojis = String::from_utf8_lossy(&gitmoji.stdout);

    for line in emojis.lines() {
        if line.contains(tag.as_str()) {
            log::debug!("Found gitmoji for tag {} {}", tag, line.to_string());
            let emoji = line.to_string().replace(tag.as_str(), "");
            return emoji;
        }
    }
    log::debug!("No gitmojis found for tag {}", tag);
    return tag;
}

pub fn replace_gitmoji(commit_message: String) -> String {
    let mut new_message = commit_message.clone();
    // Parse the string looking for unique gitmojis tags, e.g. :bug:
    let re = Regex::new(r":\w+:").unwrap();
    let matches: Vec<_> = re.find_iter(&commit_message).collect();
    // If there are no matches, return the original message
    if matches.is_empty() {
        return commit_message;
    }
    // Get the gitmojis
    for tag in matches {
        let gitmoji = get_gitmojis(tag.as_str().to_string());
        // If there are no gitmojis for the tag, skip it
        if gitmoji.is_empty() {
            continue;
        }
        // Replace the tag with the gitmoji
        new_message = new_message.replace(&tag.as_str(), &gitmoji);
    }
    return new_message;
}
