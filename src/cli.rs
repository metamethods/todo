use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub global_options: GlobalOptions,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new TODO list in the current directory
    Init(Init),

    /// Adds an item to the todo list
    Add(Add),

    /// Removes an item to the todo list
    Remove(Remove),

    /// Edits an item to the todo list
    Edit(Edit),

    /// Lists all items in the todo list
    List(List),
}

#[derive(Debug, Args)]
pub struct Init {
    /// Force an new initialization of a todo list in the current directory. Overwriting the json contents
    #[clap(long, short)]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct Add {
    pub todo: String,

    #[clap(long, short, num_args = 1.., value_delimiter = ',')]
    pub tags: Vec<String>,
}

#[derive(Debug, Args)]
pub struct Remove {
    pub index: usize,
}

#[derive(Debug, Args)]
pub struct Edit {
    pub index: usize,

    #[clap(long, short)]
    pub append: bool,

    #[clap(long, short, num_args = 1.., value_delimiter = ',')]
    pub tags: Option<Vec<String>>,

    #[clap(long, short)]
    pub completed: Option<bool>,

    #[clap(long, short = 'T')]
    pub todo: Option<String>,
}

#[derive(Debug, Args)]
pub struct List {
    #[clap(long, short)]
    pub filter: bool,

    #[clap(long, short)]
    pub show_timestamps: bool,

    #[clap(long, short)]
    pub completed: bool,

    #[clap(long, short, num_args = 1.., value_delimiter = ',')]
    pub tags: Vec<String>,
}

#[derive(Debug, Args)]
pub struct GlobalOptions {
    /// Use a global .todo.json file (which exists in the home directory)
    #[clap(long, short, global = true)]
    pub global: bool,
}
