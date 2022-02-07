use termsize;

use super::screen::Screen;
use super::super::data::category::Category;
use super::menu_entry::MenuEntry;
use std::io::{self, Read, Write, BufRead};

pub struct CategoryMenu {
    screen: Screen
}

impl CategoryMenu {
    pub fn new(screen: Screen) -> CategoryMenu {
        CategoryMenu {
            screen: screen
        }
    }
    pub fn show_categories(&self, podcasts: Vec<Category>) -> Vec<Category> {
        let result: Vec<Category> = Vec::new();
        result
    }

    // pub fn show(&self, input: &mut impl Read, output: &mut impl Write) {
    //     let count = self.entries.len();
    //     for i in 0..count {
    //         let out = self.entries[i].to_string(i + 1);
    //         println!("{}", out);
    //     }


    // }
    pub fn get_menu_input(&self, count: i32, input: &mut impl BufRead, output: &mut impl Write) -> std::io::Result<String> {


        let result = loop {
            let mut line = String::new();
            output.write("Choice: ".as_bytes())?;
            // print!("Choice: ");
            io::stdout().flush().unwrap();
    
            input.read_line(&mut line).unwrap();
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
}