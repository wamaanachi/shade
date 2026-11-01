use clap::Parser;
use crate::cli::cli::{CliArgs, Command};
use crate::cli::commands;

pub(crate) fn parse() {
    let args: CliArgs = CliArgs::parse();

    // that was panicking idk why
    let command = match args.command {
        Some(command) => command,
        None => return
    };

    match command {
        Command::Run { run_what } => {
            commands::run(run_what)
        }

        Command::List { .. } => {
            commands::list()
        }

        Command::Save { save_what } => {
            commands::save(save_what)
        }

        Command::Genshade {} => {
            commands::genshade()
        }
    }
}
