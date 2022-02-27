mod builder;
mod cli;

use std::os::unix::process::CommandExt;

use clap::Parser;

use cli::{Cli, Command, Orphans};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Install { packages } => {
            builder::install(&packages).exec();
        }
        Command::Uninstall { packages } => {
            builder::uninstall(&packages).exec();
        }
        Command::Upgrade => {
            builder::upgrade().exec();
        }
        Command::Info { package } => {
            builder::info(&package).exec();
        }
        Command::Orphans {
            cmd: Some(Orphans::Clean) | None,
        } => {
            builder::clean_orphans().exec();
        }
        Command::Orphans {
            cmd: Some(Orphans::List),
        } => {
            builder::list_orphans().exec();
        }
        Command::Completion { shell } => {
            cli::generate_completion(shell);
        }
    }
}
