mod api {
    pub mod api;
}

mod menu {
    pub mod menu_entry;
    pub mod podcast_menu;
    pub mod screen;
    pub mod simple_menu;
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
use std::io::{self, Read, Write};
use std::path::PathBuf;

use api::api::AppleSearch;
use data::podcast::Podcast;

use data::category::Category;
use rusqlite::{Connection, NO_PARAMS};

use menu::menu_entry::MenuEntry;
use menu::screen::Screen;
use menu::simple_menu::SimpleMenu;

fn main() {
    let screen = Screen::new();
    let main_menu = create_main_menu();
    let simple_menu = SimpleMenu::new(screen, main_menu);
    simple_menu.show();

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

fn trial(input: String) -> String {
    input
}

fn create_main_menu() -> Vec<MenuEntry> {
    let mut entries: Vec<MenuEntry> = Vec::new();

    entries.push(MenuEntry {
        description: String::from("Add new category"),
        f: trial,
    });

    entries.push(MenuEntry {
        description: String::from("Edit category"),
        f: trial,
    });
    entries.push(MenuEntry {
        description: String::from("Delete category"),
        f: trial,
    });

    entries
}
