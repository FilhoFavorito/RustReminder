mod cli;
//use chrono::NaiveDateTime;
use clap::Parser;
use cli::{Cli, Subcommands};

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
    let args = Cli::parse();

    match &args.subcmd {
        Subcommands::Add { message, date } => {
            let task = Task::new(message, date);
            println!("Lembrete adicionado: '{} em {}'", task.message, task.date)
        }
        _ => println!("none of the above"),
    }
}
