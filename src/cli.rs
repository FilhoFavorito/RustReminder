use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub subcmd: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Add { message: String, date: String },
    Show { id: u32 },
    Update { id: u32, new_message: String, new_date: String },
    Delete { id: u32 },
    List,
}
