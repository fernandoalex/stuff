mod cli;
mod commands;

use log::{info, LevelFilter};
use clap::Parser;
use env_logger::Builder;
use std::io::Write;

use crate::cli::Cli;

fn main() {
    // Initialize the logger with JSON output
    Builder::new()
        .format(|buf, record| {
            let json = serde_json::json!({
                "level": record.level().to_string(),
                "message": record.args().to_string(),
                "target": record.target(),
                "module_path": record.module_path(),
                "file": record.file(),
                "line": record.line(),
            });

            writeln!(buf, "{}", json)
        })
        .filter(None, LevelFilter::Info)
        .init();

    let args = Cli::parse();
    info!("Starting the application");

    match args.command {
        cli::Command::Command1 { arg1 } => {
            commands::command1::run(arg1);
        }
        cli::Command::Command2 { arg2 } => {
            commands::command2::run(arg2);
        }
    }
}
