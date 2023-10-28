use regex::Regex;
use std::process::Command;

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

pub fn replace(commit_message: String) -> String {
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
