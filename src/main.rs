use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Cli {
    List {
    },
    Add {
        #[structopt()]
        task: String,
    },
    Edit {
        #[structopt()]
        id: i32,
        #[structopt()]
        task: String,
    },
    Done {
        #[structopt()]
        id: i32,
    },
}

fn load_todos_from(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


fn print_todos(todos: Vec<String>) {
    for (index, todo) in todos.iter().enumerate() {
        println!("{} - {}", index, todo);
    }
}

fn main() {
    let todos = load_todos_from("todo.txt").expect("Could not load lines");

    // command line arguments
    match Cli::from_args() {
        Cli::Add { task } => {
            println!("New task: {}", task)
        },
        Cli::Edit { id, task } => {
            println!("Edit task: {} {}", id, task)
        },
        Cli::Done { id } => {
            println!("Done task: {}", id)
        },
        Cli::List {} => {
            print_todos(todos);
        },
    }
}
