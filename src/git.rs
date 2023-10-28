use log;
use regex::Regex;
use std::process::Command;

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

pub fn get_commits(base_branch: &str) -> Option<Vec<String>> {
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
    if commits.is_empty() {
        return None;
    }
    // Skip first line
    let commits = commits
        .lines()
        .skip(1)
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    return Some(commits);
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
