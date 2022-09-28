use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

pub fn generate_completion(shell: Shell) {
    clap_complete::generate(shell, &mut Cli::command(), BIN_NAME, &mut std::io::stdout());
}

/// Opinionated wrapper for the XBPS package manager
#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Install packages
    Install {
        /// Packages to be installed
        #[arg(value_name = "PKG", required = true)]
        packages: Vec<String>,
    },

    /// Uninstall packages
    Uninstall {
        /// Packages to be uninstalled
        #[arg(value_name = "PKG", required = true)]
        packages: Vec<String>,
    },

    /// Upgrade all installed packages
    #[command(visible_alias = "up")]
    Upgrade,

    /// Display package info
    Info {
        /// Package to display info for
        #[arg(value_name = "PKG")]
        package: String,
    },

    /// Manage orphaned packages
    Orphans {
        #[command(subcommand)]
        cmd: Option<Orphans>,
    },

    /// Generate shell completion
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Clone, Copy, Subcommand)]
pub enum Orphans {
    /// Remove orphans (default if not specified)
    Clean,

    /// List orphans
    List,
}
