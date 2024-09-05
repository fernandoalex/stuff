use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "my_cli_app", about = "A CLI app for automating work tasks")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(name = "command1")]
    Command1 {
        #[arg(short, long)]
        arg1: String,
    },
    #[command(name = "command2")]
    Command2 {
        #[arg(short, long)]
        arg2: String,
    },
}
