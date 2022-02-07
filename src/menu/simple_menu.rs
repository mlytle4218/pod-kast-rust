use super::menu_entry::MenuEntry;
use super::screen::Screen;
use std::io::{Result, Error};

use std::io::{self, Read, Write};

pub struct SimpleMenu {
    screen: Screen,
    entries: Vec<MenuEntry>,
    index: usize,
}

impl SimpleMenu {
    pub fn new(screen: Screen, entries: Vec<MenuEntry>) -> SimpleMenu {
        SimpleMenu {
            screen: screen,
            entries: entries,
            index: 0,
        }
    }
    pub fn get_menu_input(&self, count: i32) -> std::io::Result<String> {


        let result = loop {
            let mut line = String::new();
            print!("Choice: ");
            io::stdout().flush().unwrap();
    
            std::io::stdin().read_line(&mut line).unwrap();
            match line.trim().parse::<i32>() {
                Ok(val2) => {
                    if val2 <= count {
                        break val2.to_string()
                    }
                },
                Err(_) => {
                    match line.trim() {
                        "q" | "n" => break String::from(line.trim()),
                        _err => {}

                    }
                }
            }
        };
        Ok(result)
    }

    pub fn show(&self) {
        let count = self.entries.len();
        for i in 0..count {
            let out = self.entries[i].to_string(i + 1);
            println!("{}", out);
        }

        let result = self.get_menu_input(count.try_into().unwrap());
        println!("{}", result.unwrap());
        // match self.get_menu_input() {
        //     Ok(val) => println!("{}", val),
        //     Err(err) => println!("{}", err)
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_upcased_input_to_output() {
        let mut output: Vec<u8> = Vec::new();

        // upcase(&mut "Hello, world!\n".as_bytes(), &mut output).unwrap();
        // assert_eq!(&output, b"HELLO, WORLD!\n");
    }
}
