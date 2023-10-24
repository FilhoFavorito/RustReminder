use clap::{Parser, Subcommand, Args};
use chrono::NaiveDateTime;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub subcmd: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Add(AddTask),
    Show(ShowTask),
    Update(UpdateTask),
    Delete (DeleteTask),
    List,
}

#[derive(Debug, Args)]
pub struct AddTask {
    pub goal: String,
    pub creation_date: NaiveDateTime,
    pub completion_date: NaiveDateTime,
    pub finished: bool,
}

#[derive(Debug, Args)]
pub struct UpdateTask{
    pub id: i32,
    pub new_goal: String,
}

#[derive(Debug, Args)]
pub struct DeleteTask {
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct ShowTask {
    pub id: i32,
}