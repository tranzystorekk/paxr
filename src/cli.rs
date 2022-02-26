use clap::{AppSettings, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

pub fn generate_completion(shell: Shell) {
    clap_complete::generate(shell, &mut Cli::command(), BIN_NAME, &mut std::io::stdout());
}

/// Opinionated wrapper for the XBPS package manager
#[derive(Parser)]
#[clap(global_setting = AppSettings::DeriveDisplayOrder)]
#[clap(version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Install packages
    Install {
        /// Packages to be installed
        #[clap(value_name = "PKG", required = true)]
        packages: Vec<String>,
    },

    /// Uninstall packages
    Uninstall {
        /// Packages to be uninstalled
        #[clap(value_name = "PKG", required = true)]
        packages: Vec<String>,
    },

    /// Upgrade all installed packages
    #[clap(visible_alias = "up")]
    Upgrade,

    /// Display package info
    Info {
        /// Package to display info for
        #[clap(value_name = "PKG")]
        package: String,
    },

    /// Manage orphaned packages
    Orphans {
        #[clap(subcommand)]
        cmd: Option<Orphans>,
    },

    /// Generate shell completion
    Completion {
        /// Shell to generate completion for
        #[clap(arg_enum)]
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
