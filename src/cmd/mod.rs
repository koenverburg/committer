mod commit;
#[cfg(test)]
mod tests;

use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct CommitCommandArgs {
    #[arg(short, long)]
    pub preset: Option<String>,

    #[arg(short, long)]
    pub ticket: Option<String>,

    #[arg(long)]
    pub no_description: bool,

    #[arg(long)]
    pub push: bool,

    #[arg(long)]
    pub push_no_verify: bool,

    #[arg(long)]
    pub no_verify: bool,

    #[arg(long)]
    pub force_push: bool,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(name = "commit")]
    CommitCommand(CommitCommandArgs),
}

pub fn exec(cmd: Command) -> Result<()> {
    match cmd {
        Command::CommitCommand(args) => commit::exec(args),
    }
}
