use clap::Parser;
use crate::cli::cli::{CliArgs, Command};
use crate::cli::commands;

pub(crate) fn parse() {
    let args: CliArgs = CliArgs::parse();

    let command = args.command;

    match command {
        Command::Run { shader } => {
            commands::run(shader)
        }

        Command::List { .. } => {
            commands::list()
        }
        Command::Save {} => {
            commands::save()
        }
    }
}
