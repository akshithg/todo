use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Cli {
    Add {
        #[structopt()]
        task: Vec<String>,
    },
    Clear {},
    Done {
        #[structopt()]
        id: usize,
    },
    Edit {
        #[structopt()]
        id: usize,
        #[structopt()]
        task: Vec<String>,
    },
    List {},
}

fn get_todos_from(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn put_todos_to(filename: impl AsRef<Path>, todos: Vec<String>) -> io::Result<()> {
    let file = File::create(filename).unwrap();
    let mut buf = BufWriter::new(file);
    for todo in todos {
        writeln!(buf, "{}", todo).expect("Failed to write");
    }
    Ok(())
}

fn print_todos(todos: Vec<String>) {
    for (index, todo) in todos.iter().enumerate() {
        println!("{} - {}", index, todo);
    }
}

fn main() {
    let todo_file = "todo.txt";
    let todos = get_todos_from(todo_file).expect("Could not load lines");

    // command line arguments
    match Cli::from_args() {
        Cli::Add { task } => {
            let mut todos = todos;
            todos.push(task.join(" "));
            put_todos_to(todo_file, todos).unwrap();
        },
        Cli::Clear {} => {
            let todos = vec![];
            put_todos_to(todo_file, todos).unwrap();
        },
        Cli::Done { id } => {
            let mut todos = todos;
            todos.remove(id);
            put_todos_to(todo_file, todos).unwrap();
        },
        Cli::Edit { id, task } => {
            let mut todos = todos;
            todos[id] = task.join(" ");
            put_todos_to(todo_file, todos).unwrap();
        },
        Cli::List {} => {
            print_todos(todos);
        },
    }
}
