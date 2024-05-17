use crate::{
    cli::{Add, GlobalOptions},
    macros::get_todo,
    TodoItem,
};

pub fn command(command: Add, global_options: GlobalOptions) {
    let mut todo = get_todo!(global_options);

    todo.items.push(TodoItem {
        content: command.todo,
        tags: command.tags,
        completed: false,
    });
    todo.write();

    println!("Successfully added todo item into the todo list");
}
