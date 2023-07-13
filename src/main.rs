use anyhow::Result;
use app::App;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

mod api;
mod app;

fn main() -> Result<()> {
    let mut app = App::new();
    let mut rl = DefaultEditor::new()?;
    println!("Model: {}", app.get_model());
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("I: {}", line);
                let res = app.execute(line.as_str());
                println!("O: {}", res);
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
