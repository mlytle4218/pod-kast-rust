use termsize;

use super::screen::Screen;
use super::super::data::category::Category;
use super::super::data::data::DB;
use super::super::config::config::Config;
use super::menu_entry::MenuEntry;
use std::io::{self, Read, Write, BufRead};
use std::{thread, time};

use rustyline::{DefaultEditor, Result as rlResult};


pub struct CategoryMenu {
    pub screen: Screen
}

impl CategoryMenu {
    // pub fn new(screen: Screen) -> CategoryMenu {
    //     CategoryMenu {
    //         screen: screen
    //     }
    // }
    pub fn new() -> CategoryMenu {
        CategoryMenu {
            screen: Screen::new()
        }
    }
    pub fn show_categories(&self, podcasts: Vec<Category>) -> Vec<Category> {
        let result: Vec<Category> = Vec::new();
        result
    }
    pub fn create_new_category(&self) {
        let db = DB::new(Config::new());
        let conn = db.connect_to_database();
        let mut cat = Category::new();
        
        cat.name = match self.enter_info("Enter Category name",""){
            Ok(val) => val,
            Err(_) => {"".to_string()}
        };
        // let temp = cat.create_category(conn);
        // info!("temp");
        // info!("{}",temp.unwrap());
        // println!("Hello , {}", line);
        thread::sleep(time::Duration::from_secs(2));
    }
    fn enter_info(&self, message: &str, default: &str) -> rlResult<String> {
        // let mut rl = Editor::new()?;
        let mut rl = DefaultEditor::new()?;
        // let mut rl: Editor<H> = rustyline::Editor::new()?;
        // const DEFAULT_USERNAME: &str = "admin";
        let input = rl.readline_with_initial(&message, (default, ""))?;
        println!("Your selected username: {input}");
        Ok(input)
    
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

            let mut stdin = io::stdin(); // We get `Stdin` here.
    
            stdin.read_line(&mut line).unwrap();
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