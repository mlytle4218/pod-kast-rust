use log::{info,error};
use rustyline::{DefaultEditor};
use rustyline::error::ReadlineError;
use std::io::{self, Write};
use std::process;
use std::env;

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
pub fn enter_search_terms() -> std::string::String {
    println!("\x1B[2J\x1B[1;1H");
    let mut line = String::new();
    print!("Enter search terms: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    return line;
}
pub fn util_quit() {
    info!("quitting");
    process::exit(1);
}
pub fn has_flag() -> Option<String>  {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            match args[1].as_str() {
                "-u" | "--update" => return Some("update".to_string()),
                "-d" | "--download" => return Some("download".to_string()),
                "-h" | "--help" => return Some("help".to_string()),
                _ => return None
            }
        },
        _ => return None
    }
}