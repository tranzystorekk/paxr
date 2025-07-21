mod builder;
mod cli;

use std::os::unix::process::CommandExt;

use clap::Parser;

use cli::{Cli, Command, Orphans};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Install { packages } => {
            let _ = builder::install(&packages).exec();
        }
        Command::Uninstall { packages } => {
            let _ = builder::uninstall(&packages).exec();
        }
        Command::Upgrade => {
            let _ = builder::upgrade().exec();
        }
        Command::Info { package } => {
            let _ = builder::info(&package).exec();
        }
        Command::Orphans {
            cmd: Some(Orphans::Clean) | None,
        } => {
            let _ = builder::clean_orphans().exec();
        }
        Command::Orphans {
            cmd: Some(Orphans::List),
        } => {
            let _ = builder::list_orphans().exec();
        }
        Command::Search { query } => {
            let _ = builder::search(&query).exec();
        }
        Command::Completion { shell } => {
            cli::generate_completion(shell);
        }
    }
}
