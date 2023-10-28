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

    pub fn from_branch(base_branch: &str) -> Self {
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

        return CommitHistory::from_string(&commits);
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
