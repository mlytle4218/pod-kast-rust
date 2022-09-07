mod api {
    pub mod api;
}

mod menu {
    pub mod menu_entry;
    pub mod podcast_menu;
    pub mod screen;
    pub mod simple_menu;
    pub mod category_menu;
}

mod data {
    pub mod category;
    pub mod context;
    pub mod data;
    pub mod episode;
    pub mod podcast;
}
mod config {
    pub mod config;
}

use home;

use std::fs;
use std::io::{self, Read, Write, BufRead};
use std::path::PathBuf;

use api::api::AppleSearch;
use data::podcast::Podcast;

use data::category::Category;
use rusqlite::{Connection, NO_PARAMS};

use menu::menu_entry::MenuEntry;
use menu::screen::Screen;
use menu::simple_menu::SimpleMenu;

use std::{thread, time};

use std::process;

use log::{info, warn, error, LevelFilter};

use rustyline::{Editor, Result};


fn main() {
    systemd_journal_logger::init().unwrap();
    log::set_max_level(LevelFilter::Info);
    info!("logging started");
    let screen = Screen::new();
    // let main_menu = create_main_menu();
    let simple_menu = SimpleMenu::new(screen, create_main_menu());
    // let simple_menu = SimpleMenu::new(screen, main_menu);
    simple_menu.show3().unwrap();
    // simple_menu.show( &mut io::stdout()).unwrap();
    // let bob = simple_menu.show( &mut io::stdout()).unwrap();
    
    // let stdio = io::stdin();
    // let input = stdio.lock();
    // let steve = simple_menu.prompt(input, &mut io::stdout(), "Choice ");
    // simple_menu.prompt3("Choice ");


    // let mut line = String::new();
    // println!("Enter your name :");
    
    // let b1 = std::io::stdin().read_line(&mut line).unwrap();
    // steve();
    // simple_menu.show( &mut io::stdout()).unwrap();
    let config = config::config::Config::new();



    // let steve = simple_menu.prompt2(input, &mut io::stdout()).unwrap();
    // let ralph = simple_menu.prompt(input, &mut io::stdout(), &bob);
    // let mut line = String::new();
    // print!("Choice: ");
    // io::stdout().flush().unwrap();
    // match std::io::stdin().read_line(&mut line) {
    //     Ok(val) => match line.trim().parse::<i32>() {
    //         Ok(val2) => println!("{}", val2),
    //         Err(err) => println!("Parse Error: {}", err),
    //     },
    //     Err(err) => {
    //         println!("Readline error: {}", err)
    //     }
    // }




    // match std::io::stdin().read_line(&mut line) {
    //     Ok(val) => {
    //         let res = line.trim().parse::<i32>();
    //     },
    //     Err(err) => {}
    // }




    // let num = "bob".parse::<i32>();
    // match num {
    //     Ok(val) => println!("yes"),
    //     Err(err) => println!("no")
    // }
    // let mut line = String::new();
    // print!("Choice: ");
    // io::stdout().flush().unwrap();
    // let choice = std::io::stdin().read_line(&mut line).unwrap();

    // match line.trim().parse::<usize>() {
    //     Ok(val) => {
    //         let th = &main_menu[val];
    //         println!("{:?}",th);
    //     },
    //     Err(err) => println!("{}", err)
    // }
    // println!("{}", temp);

    // println!("{:?}", main_menu[0]);
    // println!("{}", &main_menu[0].to_string(1));
    // println!("{}",line);
    // let test = line.chars().all(char::is_numeric);
    // println!("{}", test);
    // let copy = line.to_owned();
    // let num = "12.4".parse::<i32>();
    // let num = copy.parse::<i32>();
    // // let num = line.parse::<f64>();
    // match num {
    //     Ok(val) => println!("Yes {}", val),
    //     Err(err) => println!("No {}", err)
    // }
}


pub fn prompt<R, W>(mut input: R, mut output: W, label: &str) -> std::string::String
where 
    R: BufRead,
    W: Write
    {
        let mut line = String::new();
        output.write(label.as_bytes()).unwrap();
        io::stdout().flush().unwrap();
        input.read_line(&mut line).unwrap();
        return line;
}

fn trial() {
    println!("trial function");
    thread::sleep(time::Duration::from_secs(2));
}
fn create_new_category() {
    let db = data::data::DB::new(config::config::Config::new());
    let conn = db.connect_to_database();
    let mut cat = Category::new();
    
    cat.name = enter_category_info();
    let temp = cat.create_category(conn);
    info!("temp");
    info!("{}",temp.unwrap());
    // println!("Hello , {}", line);
    thread::sleep(time::Duration::from_secs(2));
}

fn enter_category_info() -> std::string::String {
    println!("\x1B[2J\x1B[1;1H");
    info!("create_new_cateory");
    let mut line = String::new();
    print!("Enter Category name: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    return line;
}
fn edit_category() {
    let db = data::data::DB::new(config::config::Config::new());
    let conn = db.connect_to_database();
    let mut cat = Category::new();
    let cats = cat.read_categories(&conn);
    let result = display_cats(&cats);

    match result {
        Ok(index) => {
            match index.trim().parse::<usize>() {
                Ok(idx) => {
                    info!("ok");
                    // info!("{}", cats.unwrap()[idx-1].id);
                    info!("{}",cat.id);
                    cat.id = cats.unwrap()[idx-1].id;
                    cat.name = enter_category_info();
                    let res = cat.update_category(&conn);
                    info!("{}", cat.to_string());
                    info!("{}", res.unwrap());
                    // let mut temp_cat = cats.as_ref().unwrap()[idx-1];
                    // temp_cat.name = enter_category_info();
                    // cats.unwrap()[idx-1].update_category(&conn);
                },
                Err(e) => {}
            }
            // info!("index is {}", index.trim().parse::<i32>().unwrap() -1)
        },
        Err(e) => info!("error is {}", e)
    }
    // cats[result.trim().parse::<i32>().unwrap()-1].name = enter_category_info();
}
fn display_cats(cats: &Result<Vec<Category>>) -> Result<std::string::String> {
// fn display_cats(cats: &Result<Vec<Category>, rusqlite::Error>) -> Result<std::string::String, io::Error> {
    
    let screen = Screen::new();
    let cats_len = cats.as_ref().unwrap().len(); 
    loop {      
        println!("\x1B[2J\x1B[1;1H");
        info!("display_cats");
        for (i, ct) in cats.as_ref().unwrap().iter().enumerate() {
            // info!("{}",ct.name);
            println!("{}. {}",(i+1),ct.name);
        }
        let mut line = String::new();
        print!("Choice: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        match line.trim().parse::<i32>() {
            Ok(val) => {
                if val <= cats_len as i32  && val > 0 {
                    return Ok(val.to_string());
                }
            }
            Err(_) => {
                match line.trim() {
                    "q" | "n" => break Ok("".to_string()),
                    // "q" | "n" => return Ok(String::from(line.trim())),
                    _err => {}
                }
            }
        }
    }
    // thread::sleep(time::Duration::from_secs(2));

}

fn quit() {
    info!("quitting");
    process::exit(1);
}

fn create_main_menu() -> Vec<MenuEntry> {
    let mut entries: Vec<MenuEntry> = Vec::new();

    entries.push(MenuEntry {
        description: String::from("Add new category"),
        reference: (entries.len() + 1).to_string(),
        f: create_new_category,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Edit category"),
        reference: (entries.len() + 1).to_string(),
        f: edit_category,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Delete category"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
        entries.push(MenuEntry {
        description: String::from("Add new podcast"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Edit podcast"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("delete podcast"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("choose episodes to download"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("start downloads"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("search for podcasts"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("delete from download queue"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("update all podcasts"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("archive"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
        show: true
    });

    entries.push(MenuEntry {
        description: String::from("quit"),
        reference: "q".to_owned(),
        f: quit,
        show: false
    });

    entries
}
