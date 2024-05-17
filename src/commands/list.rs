use crate::{
    cli::{GlobalOptions, List},
    colors::{colorize, AnsiEscape},
    macros::get_todo,
};

use chrono::prelude::*;

pub fn command(command: List, global_options: GlobalOptions) {
    let todo = get_todo!(global_options);
    let completed = todo.items.iter().filter(|i| i.completed).count();

    let completed_text = colorize(
        &format!("{} / {} completed todo items", completed, todo.items.len()),
        if completed == todo.items.len() {
            AnsiEscape::Green
        } else {
            AnsiEscape::Yellow
        },
    );

    println!("{completed_text}");

    let mut filtered_item_indexes: Vec<usize> = (0..todo.items.len()).collect();

    if command.filter {
        filtered_item_indexes = filtered_item_indexes
            .into_iter()
            .filter(|i| todo.items[*i].completed == command.completed)
            .filter(|i| {
                if command.tags.is_empty() {
                    return true;
                }

                let item_tags = &todo.items[*i].tags;
                for tag in &command.tags {
                    if !item_tags.contains(tag) {
                        return false;
                    }
                }

                true
            })
            .collect::<Vec<usize>>();
    }

    for (i, item) in todo.items.iter().enumerate() {
        if command.filter && filtered_item_indexes.binary_search(&i).is_err() {
            continue;
        }

        let item_index = colorize(&(i + 1).to_string(), AnsiEscape::Blue);
        let item_content = colorize(
            &item.content,
            if item.completed {
                AnsiEscape::Strike
            } else {
                AnsiEscape::Reset
            },
        );
        let tags = colorize(&item.tags.join(", "), AnsiEscape::Magenta);
        let date_added = colorize(&Utc.timestamp_nanos(item.date_added).to_rfc2822(), AnsiEscape::Black);

        let mut string = format!("{item_index}. {item_content}");

        if command.show_timestamps {
            string.push_str(&format!(" ({date_added})"));
        }

        string.push_str(&format!(" {tags}"));

        println!("{string}");
    }
}
