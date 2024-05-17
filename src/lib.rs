use std::{
    env::current_dir,
    fs::{read_to_string, write},
    io::{Error, ErrorKind},
    path::PathBuf,
};

use cli::GlobalOptions;
use serde::{Deserialize, Serialize};

pub mod cli;
pub mod colors;
pub mod commands;

pub const TODO_FILE_NAME: &str = ".todo.json";

mod macros {
    macro_rules! get_todo {
        ($global_options:ident) => {
            crate::macros::on_err_exit!(crate::get_todo(&$global_options), |err| {
                eprintln!("{err}");
            })
        };
    }

    macro_rules! on_err_exit {
        ($result:expr, $fn:expr) => {
            match $result {
                Ok(value) => value,
                Err(err) => {
                    $fn(err);
                    std::process::exit(1);
                }
            }
        };
    }

    macro_rules! on_ok_exit {
        ($result:expr, $fn:expr) => {
            match $result {
                Ok(value) => {
                    $fn(value);
                    std::process::exit(0);
                }
                Err(err) => err,
            }
        };
    }

    pub(crate) use get_todo;
    pub(crate) use on_err_exit;
    pub(crate) use on_ok_exit;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub content: String,
    pub tags: Vec<String>,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub items: Vec<TodoItem>,

    #[serde(skip_serializing)]
    path: Option<PathBuf>,
}

impl Todo {
    pub fn empty() -> Self {
        Self {
            items: vec![],
            path: None,
        }
    }

    pub fn add_item(&mut self, todo_item: TodoItem) {
        self.items.push(todo_item);
    }

    pub fn get_item(&self, index: usize) -> Option<&TodoItem> {
        self.items.get(index)
    }

    pub fn write(&self) {
        write(
            self.path.as_ref().unwrap(),
            serde_json::to_string(self).unwrap(),
        )
        .expect("Unable to write file");
    }
}

pub fn get_home_directory() -> Result<PathBuf, Error> {
    match home::home_dir() {
        Some(path) => Ok(path),
        None => Err(Error::new(
            ErrorKind::Other,
            "Unable to get the home directory",
        )),
    }
}

pub fn get_todo(global_options: &GlobalOptions) -> Result<Todo, String> {
    let current_directory = if global_options.global {
        get_home_directory()
    } else {
        current_dir()
    };

    match current_directory {
        Ok(mut path) => {
            path.push(TODO_FILE_NAME);

            match read_to_string(&path) {
                Ok(json) => match serde_json::from_str::<Todo>(&json) {
                    Ok(mut todo) => {
                        todo.path = Some(path);
                        Ok(todo)
                    }
                    Err(_) => Err("Unable to parse file".to_string()),
                },
                Err(_) => Err(format!(
                    "Cannot find todo file {TODO_FILE_NAME}. Did you run `todo init`?"
                )),
            }
        }
        Err(_) => Err("Unable to get the current directory".to_string()),
    }
}
