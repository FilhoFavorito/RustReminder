mod cli;
//use chrono::NaiveDateTime;
use clap::Parser;
use cli::{Cli, Subcommands, AddTask, ShowTask, UpdateTask, DeleteTask};

fn main() {
    let args = Cli::parse();

    match &args.subcmd {
        Subcommands::Add(AddTask) => {
            println!("Lembrete: {:?}", AddTask)
        }
        Subcommands::Show(ShowTask) => {
            println!("Lembrete #{:?}", ShowTask)
        }
        Subcommands::Update(UpdateTask) => {
            println!("Atualizando lembrete #{:?}", UpdateTask)
        }
        Subcommands::Delete(DeleteTask) => {
            println!("Deletando lembrete #{:?}", DeleteTask)
        }
        Subcommands::List => {
            println!("Lista de lembretes")
        }
    }
}

pub fn update_task(id: u32) {

}

pub fn show_task(id: u32) {

}

pub fn show_all_tasks() {

}

pub fn delete_task(id: u32){

}