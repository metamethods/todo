use clap::Parser;
use todo::{
    cli::{Cli, Commands, List},
    commands::{add, edit, init, list, remove},
};

fn main() {
    let parsed = Cli::parse();

    let global_options = parsed.global_options;
    let command = parsed.command;

    match command {
        Some(command) => match command {
            Commands::Init(command) => init::command(command, global_options),
            Commands::Add(command) => add::command(command, global_options),
            Commands::Remove(command) => remove::command(command, global_options),
            Commands::List(command) => list::command(command, global_options),
            Commands::Edit(command) => edit::command(command, global_options),
        },
        None => list::command(
            List {
                filter: false,
                tags: vec![],
                completed: false,
                show_timestamps: false
            },
            global_options,
        ),
    }
}
