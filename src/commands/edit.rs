use crate::{
    cli::{Edit, GlobalOptions},
    macros::get_todo,
};

pub fn command(command: Edit, global_options: GlobalOptions) {
    let mut todo = get_todo!(global_options);

    if let None = todo.items.get(command.index - 1) {
        eprintln!("Todo item {} does not exist!", command.index);
        return;
    }

    let todo_item = &mut todo.items[command.index - 1];

    if let Some(todo_content) = command.todo {
        todo_item.content = todo_content;
    }

    if let Some(todo_tags) = command.tags {
        if command.append {
            todo_item.tags.extend(todo_tags);
        } else {
            todo_item.tags = todo_tags;
        }
    }

    if let Some(todo_completed) = command.completed {
        todo_item.completed = todo_completed;
    }

    todo.write();
    println!("Successfully edited todo item {}!", command.index);
}
