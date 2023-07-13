use anyhow::Result;
use app::App;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

mod api;
mod app;

fn main() -> Result<()> {
    let mut app = App::new();
    let mut rl = DefaultEditor::new()?;
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("> {}", line);
                let res = app.execute(line.as_str());
                println!("> {}", res);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctr + c");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctr + D");
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
