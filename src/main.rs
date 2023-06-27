#![deny(unused_extern_crates)]


mod api {
    pub mod api;
}

mod menu {
    pub mod menu_entry;
    pub mod screen;
    pub mod simple_menu;
}

mod data {
    pub mod category;
    pub mod data;
    pub mod episode;
    pub mod podcast;
}
mod config {
    pub mod config;
}

mod utilities {
    pub mod utilities;
}

use utilities::utilities::{util_quit, has_flag};

use data::category::Category;
use data::podcast::Podcast;
use data::episode::Episode;

use menu::menu_entry::MenuEntry;
use menu::screen::Screen;
use menu::simple_menu::SimpleMenu;

use log::{info, LevelFilter};

fn main() {
    let _config = config::config::Config::new();
    // systemd_journal_logger::init().unwrap();
    // log::set_max_level(LevelFilter::Info);
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    info!("logging started");

    
    let mut entries: Vec<MenuEntry> = Vec::new();


    entries.push(MenuEntry {
        description: String::from("Add new category"),
        reference: (entries.len() + 1).to_string(),
        f:  Category::create_category_cat,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Edit category"),
        reference: (entries.len() + 1).to_string(), 
        f: Category::edit_category_cat,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Delete category"),
        reference: (entries.len() + 1).to_string(),
        f: Category::delete_category_cat,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Add new podcast"),
        reference: (entries.len() + 1).to_string(),
        f: Podcast::create_podcast_pod,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Edit podcast"),
        reference: (entries.len() + 1).to_string(),
        f: Podcast::edit_podcast_pod,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Delete podcast"),
        reference: (entries.len() + 1).to_string(),
        f: Podcast::delete_podcast_pod,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Choose episodes to download"),
        reference: (entries.len() + 1).to_string(),
        f: Episode::choose_episodes_epi,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Start downloads"),
        reference: (entries.len() + 1).to_string(),
        f: Episode::start_downloads_epi,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Search for podcasts"),
        reference: (entries.len() + 1).to_string(),
        f: Podcast::search,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("delete from download queue"),
        reference: (entries.len() + 1).to_string(),
        f: Episode::delete_episodes_from_download_queue,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("update all podcasts"),
        reference: (entries.len() + 1).to_string(),
        f: Episode::update_episodes_for_all_podcasts,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("archive"),
        reference: (entries.len() + 1).to_string(),
        f: Podcast::archive,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("quit"),
        reference: "q".to_owned(),
        f: util_quit,
        show: false
    });

    let simple_menu = SimpleMenu::new(Screen::new(), entries);
    
    match has_flag() {
        Some(flag) =>{
            match flag.as_str() {
                "update" => Episode::command_line_update_episodes(),
                "download" => Episode::command_line_start_downloads(),
                "help" => println!("help flag"),
                _ =>  { simple_menu.show(); }
            }
        },
        None =>{
            simple_menu.show();
        }
    }
}


