use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author="jmillana", version="0.3.1", about, long_about=None)]
pub struct Cli {
    #[arg(short = 'y', global = true, help = "Auto accept the generated commit")]
    pub auto_accept: bool,

    #[arg(
        short = 'd',
        long,
        global = true,
        help = "Show the generated command without executing them"
    )]
    pub dryrun: bool,

    #[arg(
        short = 't',
        long = "token-limit",
        global = true,
        help = "Limit the ammout of tokens to be used"
    )]
    pub token_limit: Option<usize>,

    #[arg(
        short = 'H',
        long,
        help = "Drive the AI to de-genenerate commit messages that fulfill your desires"
    )]
    pub hint: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(short_flag='c', about="Autogenerate a commit message", long_about=None)]
    Commit(CommitArgs),
    #[command(short_flag='s', about="Autogenerate a squash message", long_about=None)]
    Squash(SquashArgs),
}

#[derive(Args, Debug)]
pub struct CommitArgs {
    #[arg(from_global)]
    pub auto_accept: bool,

    #[arg(from_global)]
    pub dryrun: bool,

    #[arg(from_global)]
    pub token_limit: Option<usize>,

    #[arg(from_global)]
    pub hint: Option<String>,

    #[arg(short = 'e', long, help = "Add emojis to the commits")]
    pub gitmoji: bool,
}

#[derive(Args, Debug)]
pub struct SquashArgs {
    #[arg(from_global)]
    pub auto_accept: bool,

    #[arg(from_global)]
    pub dryrun: bool,

    #[arg(from_global)]
    pub token_limit: Option<usize>,

    #[arg(short = 'e', long, help = "test")]
    pub gitmoji: bool,

    #[arg(from_global)]
    pub hint: Option<String>,

    #[arg(
        short = 'b',
        long = "base-branch",
        help = "Base branch to squash to commits into"
    )]
    pub base_branch: String,
}
