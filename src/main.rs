mod cli;
use clap::Parser;
use cli::{Cli, Commands};

#[derive(Debug)]
struct Task {
    message: String,
    date: String,
}
impl Task {
    fn new(message: &str, date: &str) -> Self {
        Self {
            message: String::from(message),
            date: String::from(date),
        }
    }
}

fn main() {
    let task = Task::new("comprar pão", "20/08/2023");
    let args = Cli::parse();

    match &args.command {
        Commands::Add { message } => {
            println!("{:?}", message)
        }
        _ => println!("{},{}", task.date, task.message),
    }
}
