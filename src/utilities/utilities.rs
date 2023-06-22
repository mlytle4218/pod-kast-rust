use log::{error};
use rustyline::{DefaultEditor};
use rustyline::error::ReadlineError;
use std::io::{self, Write};

pub fn enter_info_util(message: &str, default: &str) -> Result<String, ReadlineError> {
    match DefaultEditor::new() {
        Ok(mut rl) =>{
            loop {
                match rl.readline_with_initial(&message, (default, "")) {
                    Ok(res) => {
                        if res.len() > 0 {
                            break Ok(res)
                        } else {
                            continue
                        }
                    },
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break Err(ReadlineError::Interrupted)
                    },
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break Err(ReadlineError::Eof)
                    },
                    Err(err) => {
                        error!("Error: {:?}", err);
                        break Err(err)
                    }
                }
            }
        },
        Err(e) =>{
            Err(e)
        }
    }
}

pub fn error_message(message: &str) {
    println!("\x1B[2J\x1B[1;1H");
    error!("{}", message);
    println!("{}", message);
    print!("Press return to continue.");
    let mut line = String::new();
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
}