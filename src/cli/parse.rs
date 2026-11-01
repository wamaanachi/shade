use clap::Parser;
use crate::cli::cli::{CliArgs, Command};
use crate::cli::commands;
use crate::{app, run_demo};

pub(crate) fn parse() -> std::io::Result<()> {
    let args: CliArgs = CliArgs::parse();

    if args.command.is_none() && !args.demo {
        app::run()?;
    }

    let command = args.command.unwrap();

    match command {
        Command::Run { run_what } => {
            if run_what.is_some() {
                commands::run(run_what.unwrap())
            }
        }

        Command::List { .. } => {
            commands::list()
        }

        // Command::Save { save_what } => {
        //     commands::save(save_what)
        // }
        //
        // Command::Genshade {} => {
        //     commands::genshade()
        // }

        _ => {},
    }

    if args.demo {
        run_demo()
    }

    Ok(())
}
