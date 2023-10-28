use crate::ai::completions::chatgpty;
use crate::cli::utils::ask_for_confirmation;
use crate::cli::SquashArgs;
use crate::commands;
use crate::git;
use crate::pprint;
use crate::prompts;
use colored::Colorize;
use spinners::{Spinner, Spinners};

pub fn squash_workflow(mut chat_completions: chatgpty::GptyCompletions, args: &SquashArgs) {
    // Get the current brach
    let mut spinner = Spinner::new(Spinners::BouncingBar, "Squashing commit messages...".into());
    let auto_accept = args.auto_accept;
    let branch_name = git::get_branch_name().unwrap_or_else(|| {
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            "Failed to get branch name.".red().to_string(),
        );
        std::process::exit(1);
    });
    let base_branch = &args.base_branch;

    log::info!("Branch name: {:?}", branch_name);
    if branch_name == *base_branch {
        println!(
            "{} {}",
            "⚠️".bright_yellow(),
            "Squashing against the same branch. Unable to load commits.".bright_yellow()
        );
    }
    // Get commits ahead of the given branch
    let commit_history = git::get_commits(base_branch);
    if commit_history.commits.len() == 0 {
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            "Failed to get commits.".red().to_string(),
        );
        std::process::exit(1);
    }
    chat_completions.system_prompt = prompts::squash_system_prompt(true);
    // Generate the squash message
    let mut commits = Vec::new();
    let n_commits = commit_history.commits.len();
    for commit in commit_history.commits {
        commits.push(commit.to_string());
    }
    let prompt = prompts::get_squash_user_prompt(commits, &args.hint);
    let should_refine = !auto_accept;
    let mut commit_message = chat_completions.refine_loop(prompt, should_refine, &mut spinner);
    if args.gitmoji {
        commit_message = git::replace_gitmoji(commit_message);
    }
    if auto_accept
        || ask_for_confirmation(
            format!(">> Reset branch head to {}? [Y/n]", base_branch).as_str(),
            None,
        )
    {
        // Reset the head of the branch and commit
        pprint(&format!("git reset HEAD~{} ", n_commits), "bash");
        if args.dryrun {
            println!("Dryrun: branch head have not been modified");
        } else {
        }
    }
    if auto_accept || ask_for_confirmation(">> Squash the commits? [Y/n]", None) {
        let mut commit_cmd = "git commit -m '".to_string();
        commit_cmd.push_str(commit_message.as_str());
        commit_cmd.push_str("'");

        pprint(&commit_cmd, "bash");
        if args.dryrun {
            println!("Changes have not been applied");
            return;
        }
        let args = commands::RunArgs::build(commit_cmd);

        match commands::run(args) {
            Ok(output) => println!("{}", output.stdout),
            Err(err) => println!("{}", err),
        }
    }
}
