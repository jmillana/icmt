use crate::ai::completions::chatgpty;
use crate::cli::utils::ask_for_confirmation;
use crate::cli::CommitArgs;
use crate::commands;
use crate::git;
use crate::pprint;
use crate::prompts;
use colored::Colorize;
use spinners::{Spinner, Spinners};

pub fn commit_workflow(mut chat_completions: chatgpty::GptyCompletions, args: &CommitArgs) {
    //, mut chat_completions: completions::ChatCompletions) {
    let mut spinner = Spinner::new(
        Spinners::BouncingBar,
        "Generating your commit message...".into(),
    );
    chat_completions.system_prompt = prompts::commit_system_prompt(args.gitmoji);
    let auto_accept = args.auto_accept;
    println!("auto accept {}", auto_accept);

    let commit_changes = git::get_commit_changes().unwrap_or_else(|| {
        spinner.stop_and_persist(
            "✖".red().to_string().as_str(),
            "Failed to get commit changes.".red().to_string(),
        );
        std::process::exit(1);
    });

    let prompt = prompts::get_commit_user_prompt(commit_changes, &args.hint);
    let should_refine = !auto_accept;
    let mut commit_message = chat_completions.refine_loop(prompt, should_refine, &mut spinner);

    if args.gitmoji {
        commit_message = git::replace_gitmoji(commit_message);
    }

    if auto_accept || ask_for_confirmation(">> Apply the generated commit? [Y/n]", None) {
        let mut commit_cmd = "git commit -m '".to_string();
        commit_cmd.push_str(commit_message.as_str());
        commit_cmd.push_str("'");

        pprint(&commit_cmd, "bash");
        if args.dryrun {
            println!("Changes have not been applied");
            return;
        }
        let mut args = commands::RunArgs::build(commit_cmd);
        args.spinner_flag = true;

        match commands::run(args) {
            Ok(output) => println!("{}", output.stdout),
            Err(err) => println!("{}", err),
        }
    }
}
