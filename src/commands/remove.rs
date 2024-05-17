use crate::{
    cli::{GlobalOptions, Remove},
    macros::get_todo,
};

pub fn command(command: Remove, global_options: GlobalOptions) {
    let mut todo = get_todo!(global_options);

    if let None = todo.items.get(command.index - 1) {
        eprintln!("Todo item {} does not exist!", command.index);
        return;
    }

    todo.items.remove(command.index - 1);
    todo.write();

    println!("Removed todo item {}!", command.index);
}
