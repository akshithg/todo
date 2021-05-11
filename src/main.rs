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

fn main() {
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
            println!("list of tasks")
        },
    }
}
