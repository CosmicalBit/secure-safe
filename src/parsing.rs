use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ParsedArgs {
    Restore(String),
    WatchDir(PathBuf),
    Delete(String),
    Add(PathBuf),
    About,
}

#[derive(Parser)]
#[command(name = "secure_safe", about = "A fully local encrypted-file vault")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { path: PathBuf },
    Delete { path: String },
    Restore { name: String },
    About,
    Watchd { directory: PathBuf },
}

pub fn parse() -> ParsedArgs {
    match Cli::parse().command {
        Command::Add { path } => ParsedArgs::Add(path),
        Command::Delete { path } => ParsedArgs::Delete(path),
        Command::Restore { name } => ParsedArgs::Restore(name),
        Command::About => ParsedArgs::About,
        Command::Watchd { directory } => ParsedArgs::WatchDir(directory),
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Cli, Command};

    #[test]
    fn parses_each_command() {
        let cli = Cli::try_parse_from(["secure_safe", "add", "/tmp/secret.txt"]).unwrap();
        assert!(matches!(cli.command, Command::Add { .. }));

        let cli = Cli::try_parse_from(["secure_safe", "about"]).unwrap();
        assert!(matches!(cli.command, Command::About));
    }

    #[test]
    fn rejects_missing_required_argument() {
        assert!(Cli::try_parse_from(["secure_safe", "restore"]).is_err());
    }
}
