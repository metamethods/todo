use std::env::current_dir;

use crate::{
    cli::{GlobalOptions, Init},
    get_home_directory, get_todo,
    macros::on_ok_exit,
    Todo, TODO_FILE_NAME,
};

pub fn command(command: Init, global_options: GlobalOptions) {
    if !command.force {
        on_ok_exit!(get_todo(&global_options), |_| {
            eprintln!("A todo file already exists in this directory!");
        });
    }

    let mut path = if global_options.global {
        get_home_directory()
    } else {
        current_dir()
    }
    .unwrap();
    path.push(TODO_FILE_NAME);

    let mut todo = Todo::empty();
    todo.path = Some(path.clone());
    todo.write();

    println!(
        "Created a new {TODO_FILE_NAME} in {}",
        path.to_string_lossy()
    );
}
