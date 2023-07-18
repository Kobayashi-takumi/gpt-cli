use anyhow::Result;
use app::App;
use clap::{Parser, Subcommand};
use loader::{initialize_config_file, initialize_log_file, log_file};
use logger::setup;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

mod api;
mod app;
mod loader;
mod logger;

#[derive(Parser)]
#[clap(
    name = "GPT CLI",
    author = "Takumi Kobayashi",
    version = "v0.5.0",
    about = "This application can call the OpenAI API from the terminal."
)]
struct AppArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, PartialEq, Clone, Subcommand)]
pub enum Command {
    Chat,
    Image,
}

fn main() -> Result<()> {
    let cli = AppArgs::parse();
    initialize_log_file()?;
    initialize_config_file()?;
    let log_file = log_file()?;
    setup(&log_file)?;
    let mut app = App::new(cli.command)?;
    let mut rl = DefaultEditor::new()?;
    println!("Model: {}", app.get_model());
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                println!("I: {}", line);
                match app.execute(line.as_str()) {
                    Ok(res) => {
                        println!("O: {}", res);
                    }
                    Err(e) => {
                        println!("Error: {}", e.to_string());
                    }
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("See you...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("See you...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
