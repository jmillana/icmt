use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author="jmillana", version="v0.2.0", about, long_about=None)]
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

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(short_flag='c', about="Autogenerate a commit message", long_about=None)]
    Commit(CommitArgs),
}

#[derive(Args, Debug)]
pub struct CommitArgs {
    #[arg(from_global)]
    pub auto_accept: bool,
    #[arg(from_global)]
    pub dryrun: bool,
    #[arg(from_global)]
    pub token_limit: Option<usize>,

    #[arg(short = 'e', long, help = "test")]
    pub gitmoji: bool,
    #[arg(short = 'H', long, help = "hint")]
    pub hint: Option<String>,
}
