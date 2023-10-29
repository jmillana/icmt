use std::process::Command;

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
