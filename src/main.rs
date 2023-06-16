#![deny(unused_extern_crates)]


mod api {
    pub mod api;
    pub mod retrieve;
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

use std::cmp::min;
use std::fs::File;

use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;

use std::io::{self, Write, BufRead};
use api::api::AppleSearch;
use api::retrieve::Retreive;

use data::category::Category;
use data::podcast::Podcast;
use data::episode::Episode;

use menu::menu_entry::MenuEntry;
use menu::screen::Screen;
use menu::simple_menu::SimpleMenu;



use std::{thread, time};

use std::process;

use log::{info, error};
use rustyline::{DefaultEditor};
use rusqlite::{Error};
use rustyline::error::ReadlineError;


use reqwest::{Client, Error as ReqError};

use std::collections::HashSet;

use std::path::Path;

fn main() {
    // systemd_journal_logger::init().unwrap();
    // log::set_max_level(LevelFilter::Info);
    
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    info!("logging started");

    
    let mut entries: Vec<MenuEntry> = Vec::new();

    entries.push(MenuEntry {
        description: String::from("Add new category"),
        reference: (entries.len() + 1).to_string(),
        // f: cat_name.create_new_category,
        // f: category_menu.create_new_category(),
        f: create_new_category,
        // f: CategoryMenu::create_new_category(&category_menu),
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
        f: delete_category,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Add new podcast"),
        reference: (entries.len() + 1).to_string(),
        f: create_podcast,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Edit podcast"),
        reference: (entries.len() + 1).to_string(),
        f: edit_podcast,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Delete podcast"),
        reference: (entries.len() + 1).to_string(),
        f: delete_podcast,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Choose episodes to download"),
        reference: (entries.len() + 1).to_string(),
        f: choose_episodes,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Start downloads"),
        reference: (entries.len() + 1).to_string(),
        f: start_downloads,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Search for podcasts"),
        reference: (entries.len() + 1).to_string(),
        f: search,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("delete from download queue"),
        reference: (entries.len() + 1).to_string(),
        f: delete_from_download_queue,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("update all podcasts"),
        reference: (entries.len() + 1).to_string(),
        f: download_latest_episode_data_for_podcast,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("archive"),
        reference: (entries.len() + 1).to_string(),
        f: archive,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("quit"),
        reference: "q".to_owned(),
        f: quit,
        show: false
    });




    let _config = config::config::Config::new();
    let simple_menu = SimpleMenu::new(Screen::new(), entries);
    simple_menu.show4();
}


// fn trial() {
//     println!("trial function");
//     thread::sleep(time::Duration::from_secs(2));
// }
fn create_new_category() {
    println!("\x1B[2J\x1B[1;1H");
    let mut cat = Category::new();
    match enter_category_info("Enter Category name ","") {
        Ok(result) =>{ 
            cat.name = result;
            match cat.create_exisitng() {
                Ok(db_response) =>{
                    info!("Category {} created", cat.name);
                },
                Err(e) => {
                    error!("{}", e);
                }
            }
        },
        Err(e) => {
            error!("{}", e);
        }
    }
}
fn enter_category_info(message: &str, default: &str) -> Result<std::string::String, ReadlineError> {
    let mut rl = DefaultEditor::new()?;
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

}
fn edit_category() {
    let mut cat = Category::new();
    match cat.read_all_categories() {
        Ok(cats) =>{
            match display_cats4(cats) {
                Ok(mut chosen_cat) =>{
                    // convert this later to return error if they enter nothing and circumvent this check
                    let res_name = enter_category_info("Existing Category name: ",&chosen_cat.name).unwrap();
                    if res_name.len() > 0 {
                        chosen_cat.name = res_name.to_string();
                        match chosen_cat.update_existing() {
                            Ok(_) => {
                                info!("{} updated", chosen_cat.name);
                            },
                            Err(_) => {
                                error!("{} could not be updated", chosen_cat.name);
                                error_message("Could not update the Category.");
                            }
                        }
                    } else {
                        // nothing was entered - don't update
                    }
                },
                Err(e) =>{
                    error!("{}", e);
                }
            }
        },
        Err(e) =>{
            error!("{}", e);
        }
    }
}
fn delete_category() {
    let mut cat = Category::new();
    match cat.read_all_categories() {
        Ok(cats) =>{
            match display_cats4(cats) {
                Ok(mut chosen_cat) =>{
                    match chosen_cat.delete_existing() {
                        Ok(_) => {
                            info!("{} deleted", chosen_cat.name);
                        },
                        Err(_) => {
                            error!("{} could not be deleted", chosen_cat.name);
                            error_message("Could not delete the Category.");
                        }
                    }
                },
                Err(e) =>{
                    error!("{}", e);
                }
            }
        },
        Err(e) =>{
            error!("{}", e);
        }
    }
}
fn create_podcast() {
    println!("\x1B[2J\x1B[1;1H");
    match edit_podcast_details2(Podcast::new()) {
        Ok(mut podcast) =>{
            match podcast.save_existing() {
                Ok(res) =>{
                    info!("Podcast {} created", podcast.name);
                },
                Err(e) =>{
                    error!("{}", e)
                }
            }
        },
        Err(e) =>{
            error!("{}", e)
        }
    }
}
fn edit_podcast() {
    println!("\x1B[2J\x1B[1;1H");
    match Podcast::new().read_all_podcasts() {
        Ok(res) =>{
            match display_pods_single_result(&res) {   
                Ok(chosen) =>{
                    match edit_podcast_details2(res[(chosen as usize)-1].clone()) {
                        Ok(mut podcast) =>{
                            match podcast.update_existing() {
                                Ok(response) => {
                                    info!("{} updated", podcast.name)
                                },
                                Err(e) =>{
                                    error!("{}", e);
                                }
                            }
                        },
                        Err(e) =>{
                            error!("{}", e);
                            error_message("Coudl not save Podcast details");
                        }
                    }
                },
                Err(e) => {
                    error!("{}",e);
                }
            }
        },
        Err(e) => {
            error!("{}",e);
        }
    }

}
fn delete_podcast() {
    match Podcast::new().read_all_podcasts() {
        Ok(mut pods ) => {
            match display_pods2(&pods) {
                Ok(chosen) => {
                    for mut podcast in chosen {
                        match podcast.delete_existing() {
                            Ok(res) => {
                                info!("{} was deleted", podcast.name);
                            },
                            Err(e) =>{
                                error!("{}", e);
                            }
                        }
                    }
                },
                Err(e) => {
                    error!("{}", e);
                }
            }
        },
        Err(e) =>{
            error!("{}", e);
        }
    }
}
fn choose_episodes() {
    match Category::new().read_all_categories() {
        Ok(cats) => {
            match display_cats3(cats) {
                Ok(c) => {
                    // maybe return c as an Option and cut out all this crap?
                    let pod: Podcast = Podcast::new();
                    let category: Option<String>;
                    if (c.len() == 0) {
                        category = None;
                    } else {
                        category = Some(c);
                    }
                    match pod.read_all_podcasts2(category) {
                        Ok(pods) =>{
                            loop {
                                match display_pods_to_choose_episodes(&pods) {
                                    Ok(_chosen) => {
                                        // nada
                                    },
                                    Err(e) => {
                                        error!("{}", e);
                                        break
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            error!("{}", e)
                        }
                    }
                },
                Err(e) => {
                    error!("{}", e)
                }
            }
        },
        Err(e) => {
            error!("{}", e)
        }
    }
}











fn display_pods2(pods: &Vec<Podcast>) -> Result<Vec<Podcast>, Error> {
    let screen = Screen::new();
    let pods_len = pods.len(); 
    let mut results: Vec<Podcast> = Vec::new();
    if pods.len() == 0 {
        error_message(format!("No Podcasts to display.").as_str());
        return Ok(results)
    }
    let display_size: u16 = screen.row_size -1;
    let mut pages: u16 = 0;
    let mut page_iter = 0;
    let mut row_iter; // = 0;
    if (pods_len as u16).rem_euclid(display_size) > 0 {
        pages += 1;
        pages += (pods_len as u16)/(display_size);
    }
    
    loop {
        println!("\x1B[2J\x1B[1;1H");
        info!("Display pods");
        let start = page_iter*display_size;
        let end; // = 0;

        if ((page_iter+1)*display_size)-1 < (pods_len as u16) - 1 {
            end = ((page_iter+1)*display_size)-1;
        } else {
            end = (pods_len as u16) - 1;
        }

        row_iter = start;
        while row_iter <= end {
            println!("{}. {}", (row_iter + 1), pods[row_iter as usize].name);
            row_iter += 1;
        }
        let mut line = String::new();
        print!("Choice: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line);
        match line.trim_end_matches('\n') {
            "q" => break Ok(results),
            "n" => {
                if page_iter < (pages -1) {
                    page_iter += 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            "p" => {
                if page_iter > 0 {
                    page_iter -= 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            _ => {
                let all: Vec<&str> = line.trim_end_matches('\n').split(",").collect();
                for each in all {
                    // info!("{}",each);
                    if each.contains("-") {
                        info!("has a dash" );
                        let dash: Vec<&str> = each.split("-").collect();
                        if dash.len() > 2 {
                            error_message(format!("{each} is formatted incorrectly").as_str());
                            break
                        } else {
                            match dash[0].parse::<u16>() {
                                Ok(val) => {
                                    match (dash[1]).parse::<u16>() {
                                        Ok(val2) =>{
                                            if val >= (start + 1) && val2 <= (end + 1) {
                                                for v in val..=val2 {
                                                    info!("{}", v);
                                                    results.insert( 0 as usize, pods[(v as usize)-1].clone());
                                                }
                                            }
                                        },
                                        Err(_) => {
                                            let temp = dash[1];
                                            error_message(format!("{temp} is not a valid nubmer.").as_str());
                                        }
                                    }
                                },
                                Err(_) => {
                                    let temp = dash[0];
                                    error_message(format!("{temp} is not a valid nubmer.").as_str());
                                }
                            }
                        }
                    } else  {
                        match each.parse::<u16>() {
                            Ok(val) => {
                                results.insert( 0 as usize, pods[(val as usize)-1].clone());
                            },
                            Err(_) => {
                                error_message(format!("{} is not a valid number.", each).as_str())
                            }
                        }
                    }

                }
            }
        }
    }
}














// fn delete_podcast2() {
//     match Podcast::new().read_all_podcasts() {
//         Ok(mut pods ) => {
//             match display_pods(&pods) {
//                 Ok(chosen) => {
//                     info!("delete_podcast chosen.len(): {}", chosen.len());
//                     if chosen.len() > 0 {
//                         let mut v: Vec<&u16> = chosen.iter().collect();
//                         v.sort();
//                         info!("delete_podcast v: {:?}", v);
//                         for each in v {
//                             info!("each: {}", (*each as usize)-1);
//                             info!("Podcast returned {:?}",pods[(*each as usize)-1]);
//                             match pods[(*each as usize)-1].delete_existing() {
//                                 Ok(pods) => {
//                                     info!("Ok pods: {:?}", pods);
//                                 },
//                                 Err(e) => {
//                                     error!("{}", e);
//                                 }
//                             }
                            
//                         }
//                     } else {
//                         // quit without choosing
//                     }
//                 },
//                 Err(e) => {
//                     error!("{}", e);
//                 }
//             }
//         },
//         Err(e) =>{
//             error!("{}", e);
//         }
//     }
// }
// fn delete_category2()  {
//     let db = data::data::DB::new(config::config::Config::new());
//     let conn = db.connect_to_database();
//     let mut cat = Category::new();
//     let cats = cat.read_categories(&conn);
//     let result = display_cats2(&cats);
//     match result {
//         Ok(index) => {
//             match index.trim().parse::<usize>() {
//                 Ok(idx) => {
//                     cat.id = cats.unwrap()[idx-1].id;
//                     let res = cat.delete_category(conn);
//                     info!("{}", res.unwrap());
//                 },
//                 Err(e) => error!("delete_category() error is {}", e)
//             }
//         },
//         Err(e) => error!("delete_category() error is {}", e)
//     }

// }


fn enter_search_terms() -> std::string::String {
    println!("\x1B[2J\x1B[1;1H");
    info!("enter_search_terms");
    let mut line = String::new();
    print!("Enter search terms: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    return line;
}


fn error_message(message: &str) {
    println!("\x1B[2J\x1B[1;1H");
    error!("{}", message);
    println!("{}", message);
    print!("Press return to continue.");
    let mut line = String::new();
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line);
}

// fn edit_category2() {
//     let db = data::data::DB::new(config::config::Config::new());
//     let conn = db.connect_to_database();
//     let mut cat = Category::new();
//     let cats = cat.read_categories(&conn);
//     let result = display_cats2(&cats);

//     match result {
//         Ok(index) => {
//             info!("index :{:?}", index);
//             match index.trim().parse::<usize>() {
//                 Ok(idx) => {
//                     cat.id = cats.unwrap()[idx-1].id;
//                     let res2 = cat.read_category_by_id(&conn, idx);
//                     match res2 {
//                         Ok(temp_cat) =>{
//                             info!("temp_cat.id:{:?}", temp_cat.id);
//                             let res_name = enter_category_info("Existing Category name: ",&temp_cat.name).unwrap();
//                             if res_name.len() > 0 {
//                                 cat.name = res_name.to_string();
//                                 let update = cat.update_category(&conn);
//                                 match update {
//                                     Ok(_) => {},
//                                     Err(_) => { error_message("Could not update the Category.")}
//                                 }
//                             } else {
//                                 // nothing was entered - don't update
//                             }
//                         },
//                         Err(e) => error_message(&format!("edit_category() error is {}", e))
//                     }

//                 },
//                 Err(_e) =>  {
//                     match index.as_str() {
//                         "q" => {},
//                         _ => error_message(&format!("2edit_category() error is {:?}", index))

//                     }
//                 }
//             }
//         },
//         Err(e) => error_message(&format!("edit_category() error is {}", e))
//     }
// }
fn search() {
    let terms = enter_search_terms();
    let search = AppleSearch::new("https://itunes.apple.com".to_string(),terms.to_string(),100);
    let results = search.search();
    match results {
        Ok(mut res) =>{
            // info!("{:?}", results)
            match display_pods(&res) {
                Ok(chosen) =>{
                    info!("Ok(chosen");
                    let mut v: Vec<&u16> = chosen.iter().collect();
                    v.sort();
                    for each in v {
                        info!("Podcast returned {:?}",res[*each as usize]);
                        match res[*each as usize].save_existing() {
                            Ok(tmp) => {
                                info!("Ok tmp: {}", tmp);
                            },
                            Err(e) => {
                                error!("{}", e);
                            }
                        }
                        
                    }
                },
                Err(_) => {
                    error_message(format!("Had an issue returing the podcasts").as_str())
                }
            }
        },
        Err(_) => {}
    }
}

fn edit_podcast2() {
    println!("\x1B[2J\x1B[1;1H");
    let pod: Podcast = Podcast::new();
    match pod.read_all_podcasts() {
        Ok(res) =>{
            match display_pods_single_result(&res) {   
                Ok(chosen) =>{
                    info!("Ok(chosen");
                    match edit_podcast_details2(res[(chosen as usize)-1].clone()) {
                        Ok(mut podcast) =>{
                            match podcast.update_existing() {
                                Ok(response) => {
                                    info!("{} updated", podcast.name)
                                },
                                Err(e) =>{
                                    error!("{}", e);
                                }
                            }
                        },
                        Err(e) =>{
                            error!("{}", e);
                        }
                    }
                },
                Err(_) => {
                    // error_message(format!("Had an issue returing the podcasts-inner").as_str())
                }
            }
        },
        Err(_) => {
            error_message(format!("Had an issue returing the podcasts-outer").as_str())
        }
    }

}
fn edit_podcast_details(mut pod: Podcast) {
    match enter_category_info("Podcast name: ", &pod.name) {
        Ok(name) => {
            pod.name = name;
            info!("New Podcast name: {}", pod.name);
            match enter_category_info("Podcast URL: ", &pod.url) {
                Ok(url) => {
                    pod.url = url;
                    info!("New Podcast URL: {}", pod.url);
                    match enter_category_info("Default Audio Save location : ", &pod.audio) {
                        Ok(audio) =>{
                            pod.audio = audio;
                            info!("New Podcast default audio location: {}", pod.audio);
                            match enter_category_info("Default Audio Save location : ", &pod.video) {
                                Ok(video) => {
                                    pod.video = video;
                                    info!("New Podcast default audio location: {}", pod.video);
                                    match pod.save_existing() {
                                        Ok(tmp) => {
                                            info!("Ok tmp: {}", tmp);
                                        },
                                        Err(e) => {
                                            error!("{}", e);
                                        }
                                    }
                                }, Err(e) =>{
                                    error!("create_podcast-video: {}", e);
                                }
                            }
                        }, Err(e) =>{
                            error!("create_podcast-audio: {}", e);
                        }
                    }
                },
                Err(e) =>{
                    error!("create_podcast-url: {}", e);
                }
            }
        },
        Err(e) => {
            error!("create_podcast-name: {}", e);
        }
    }

}
fn edit_podcast_details2(mut pod: Podcast) -> Result<Podcast, ReadlineError> {
    match enter_category_info("Podcast name: ", &pod.name) {
        Ok(name) => {
            pod.name = name;
            info!("New Podcast name: {}", pod.name);
            match enter_category_info("Podcast URL: ", &pod.url) {
                Ok(url) => {
                    pod.url = url;
                    info!("New Podcast URL: {}", pod.url);
                    match enter_category_info("Default Audio Save location : ", &pod.audio) {
                        Ok(audio) =>{
                            pod.audio = audio;
                            info!("New Podcast default audio location: {}", pod.audio);
                            match enter_category_info("Default Audio Save location : ", &pod.video) {
                                Ok(video) => {
                                    pod.video = video;
                                    info!("New Podcast default audio location: {}", pod.video);
                                    return Ok(pod)
                                }, Err(e) =>{
                                    error!("create_podcast-video: {}", e);
                                    return Err(e)
                                }
                            }
                        }, Err(e) =>{
                            error!("create_podcast-audio: {}", e);
                            return Err(e)
                        }
                    }
                },
                Err(e) =>{
                    error!("create_podcast-url: {}", e);
                    return Err(e)
                }
            }
        },
        Err(e) => {
            error!("create_podcast-name: {}", e);
            return Err(e)
        }
    }

}

fn display_pods(pods: &Vec<Podcast>) -> Result<HashSet<u16>, Error> {
    let screen = Screen::new();
    let pods_len = pods.len(); 
    let mut results: HashSet<u16> = HashSet::new();
    if pods.len() == 0 {
        error_message(format!("No Podcasts to display.").as_str());
        return Ok(results)
    }
    let display_size: u16 = screen.row_size -1;
    let mut pages: u16 = 0;
    let mut page_iter = 0;
    let mut row_iter; // = 0;
    if (pods_len as u16).rem_euclid(display_size) > 0 {
        pages += 1;
        pages += (pods_len as u16)/(display_size);
    }
    
    loop {
        println!("\x1B[2J\x1B[1;1H");
        info!("Display pods");
        let start = page_iter*display_size;
        let end; // = 0;

        if ((page_iter+1)*display_size)-1 < (pods_len as u16) - 1 {
            end = ((page_iter+1)*display_size)-1;
        } else {
            end = (pods_len as u16) - 1;
        }

        row_iter = start;
        while row_iter <= end {
            println!("{}. {}", (row_iter + 1), pods[row_iter as usize].name);
            row_iter += 1;
        }
        let mut line = String::new();
        print!("Choice: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line);
        match line.trim_end_matches('\n') {
            "q" => break Ok(results),
            "n" => {
                if page_iter < (pages -1) {
                    page_iter += 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            "p" => {
                if page_iter > 0 {
                    page_iter -= 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            _ => {
                info!("display_pods not q, n, or p");
                let all: Vec<&str> = line.trim_end_matches('\n').split(",").collect();
                for each in all {
                    // info!("{}",each);
                    if each.contains("-") {
                        info!("has a dash" );
                        let dash: Vec<&str> = each.split("-").collect();
                        if dash.len() > 2 {
                            error_message(format!("{each} is formatted incorrectly").as_str());
                            break
                        } else {
                            match dash[0].parse::<u16>() {
                                Ok(val) => {
                                    match (dash[1]).parse::<u16>() {
                                        Ok(val2) =>{
                                            if val >= (start + 1) && val2 <= (end + 1) {
                                                for v in val..=val2 {
                                                    info!("{}", v);
                                                    results.insert(v);
                                                }
                                            }
                                        },
                                        Err(_) => {
                                            let temp = dash[1];
                                            error_message(format!("{temp} is not a valid nubmer.").as_str());
                                        }
                                    }
                                },
                                Err(_) => {
                                    let temp = dash[0];
                                    error_message(format!("{temp} is not a valid nubmer.").as_str());
                                }
                            }
                        }
                    } else  {
                        match each.parse::<u16>() {
                            Ok(val) => {
                                results.insert(val);
                            },
                            Err(_) => {
                                error_message(format!("{} is not a valid number.", each).as_str())
                            }
                        }
                    }

                }
            }
        }
    }
}
fn display_pods_single_result(pods: &Vec<Podcast>) -> Result<u16, Error> {
    let screen = Screen::new();
    let pods_len = pods.len(); 
    let mut results: u16;
    if pods.len() == 0 {
        error_message(format!("No Podcasts to display.").as_str());
        return Err((Error::InvalidColumnName("".to_string())))
    }
    let display_size: u16 = screen.row_size -1;
    let mut pages: u16 = 0;
    let mut page_iter = 0;
    let mut row_iter; // = 0;
    if (pods_len as u16).rem_euclid(display_size) > 0 {
        pages += 1;
        pages += (pods_len as u16)/(display_size);
    }
    
    loop {
        println!("\x1B[2J\x1B[1;1H");
        info!("Display pods");
        let start = page_iter*display_size;
        let end; // = 0;

        if ((page_iter+1)*display_size)-1 < (pods_len as u16) - 1 {
            end = ((page_iter+1)*display_size)-1;
        } else {
            end = (pods_len as u16) - 1;
        }

        row_iter = start;
        while row_iter <= end {
            println!("{}. {}", (row_iter + 1), pods[row_iter as usize].name);
            row_iter += 1;
        }
        let mut line = String::new();
        print!("Choice: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line);
        match line.trim_end_matches('\n') {
            "q" => break return Err((Error::InvalidColumnName("".to_string()))),
            "n" => {
                if page_iter < (pages -1) {
                    page_iter += 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            "p" => {
                if page_iter > 0 {
                    page_iter -= 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            _ => {
                info!("display_pods not q, n, or p");
                match line.trim_end_matches('\n').parse::<u16>() {
                    Ok(val) => {
                        return Ok(val)
                    },
                    Err(_) => {
                        error_message(format!("{} is not a valid number.", line.trim()).as_str())
                    }
                }
            }
        }
    }
}
fn display_pods_to_choose_episodes(pods: &Vec<Podcast>) -> Result<i16, Error> {
    let screen = Screen::new();
    let pods_len = pods.len(); 
    let result: i16 = -1;
    if pods.len() == 0 {
        error_message(format!("No Podcasts to display.").as_str());
        return Ok(result)
    }
    let display_size: u16 = screen.row_size -1;
    let mut pages: u16 = 0;
    let mut page_iter = 0;
    let mut row_iter; // = 0;
    if (pods_len as u16).rem_euclid(display_size) > 0 {
        pages += 1;
        pages += (pods_len as u16)/(display_size);
    }
    
    loop {
        println!("\x1B[2J\x1B[1;1H");
        info!("Display pods");
        let start = page_iter*display_size;
        let end; // = 0;

        if ((page_iter+1)*display_size)-1 < (pods_len as u16) - 1 {
            end = ((page_iter+1)*display_size)-1;
        } else {
            end = (pods_len as u16) - 1;
        }

        row_iter = start;
        
        let epi_temp: Episode = Episode::new();
        while row_iter <= end {
            match epi_temp.count_episodes(pods[row_iter as usize].id) {
                Ok(count) => {
                    println!("{}. {} - {}", (row_iter + 1), pods[row_iter as usize].name, count);
                    row_iter += 1;
                },
                Err(e) => {
                    error!("{}",e)
                }

            }
        }
        let mut line = String::new();
        print!("Choice: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line);
        match line.trim_end_matches('\n') {
            "q" => break Ok(result),
            "n" => {
                if page_iter < (pages -1) {
                    page_iter += 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            "p" => {
                if page_iter > 0 {
                    page_iter -= 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            _ => {
                info!("display_pods not q, n, or p");
                match line.trim().parse::<i16>() {
                    Ok(line_parsed) => {
                        let episode: Episode = Episode::new();
                        match episode.read_all_episodes_by_podcast_id(pods[(line_parsed as usize)-1].id, None) {
                            Ok(episodes) =>{
                                match display_episodes(&episodes) {
                                    Ok(chosen_epis) => {
                                        info!("{:?}", chosen_epis);
                                        add_episodes_to_download_queue(chosen_epis);

                                    },
                                    Err(e) => {
                                        error!("{}", e)
                                    }
                                }
                            },
                            Err(e) => {
                                error!("{}", e)
                            }
                        }
                    },
                    Err(_) => {
                        error_message(format!("{} is not a valid number.", line.trim()).as_str())
                    }

                }
            }
        }
    }
}

fn add_episodes_to_download_queue(episodes: Vec<Episode>) {
    for episode in episodes {
        match episode.add_to_download_queue() {
            Ok(result) =>{
                info!("{}",result);
                // all good in the hood
            }, 
            Err(e) =>{
                error!("{}", e);
            }
        }
    }
}

fn delete_from_download_queue() {
    match Episode::read_all_in_download_queue() {
        Ok(episodes) =>{
            info!("{:?}", episodes);
            match display_episodes(&episodes) {
                Ok(epis_to_be_removed) => {
                    for epi in epis_to_be_removed {
                        match epi.remove_from_download_queue() {
                            Ok(result) =>{
                                info!("{} removed", result);
                            },
                            Err(e) => {
                                error!("{}", e);
                            }
                        }
                    }
                },
                Err(e) => {
                    error!("{}", e);
                }
            }
        },
        Err(e) => {
            error!("{}", e);
        }
    }
}

fn download_latest_episode_data_for_podcast() {
    let retrieve = Retreive::new();
    
    let temp_pod: Podcast = Podcast::new();
    match temp_pod.read_all_podcasts() {
        Ok(pods)=>{
            for pod in pods {
                info!("{}, {}", pod.url, pod.id);
                println!("Checking episodes for {}", pod.name);
                match retrieve.retreive_episodes(pod.url, pod.id as i16) {
                    Ok(episodes) =>{
                        info!("{:?}", episodes.len());
                        for mut episode in episodes {
                            info!("{}", episode.title);
                            match episode.save_existing() {
                                Ok(_res) => {
                                    info!("Episode {} added", episode.title);
                                },
                                Err(e) =>{
                                    error!("{}", e);
                                }
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}",e)
                    }
                }
            }
        },
        Err(e) =>{
            error!("{}", e)
        }
    }
    

}

fn archive() {
    match Podcast::new().read_all_podcasts() {
        Ok(pods) =>{
            loop {
                match display_pods_to_choose_episodes_archive2(&pods) {
                    Ok(chosen) => {
                        match Episode::new().read_all_episodes_by_podcast_id(chosen.id, Some(1 as i64)) {
                            Ok(all_episodes) =>{
                                match display_episodes(&all_episodes) {
                                    Ok(chosen_episodes) =>{
                                        for episode in chosen_episodes {
                                            match episode.add_to_download_queue() {
                                                Ok(res) =>{
                                                    info!("{} was added to download queue", episode.title)
                                                },
                                                Err(e) =>{
                                                    error!("{}", e);
                                                }
                                            }
                                        }
                                    },
                                    Err(e) =>{
                                        error!("{}", e);
                                    }
                                }
                            },
                            Err(e) =>{
                                error!("{}", e);
                            }
                        }
                        // match display_episodes(chosen) {
                        //     Ok(episodes) => {
                        //         for episode in episodes {
                        //             match episode.add_to_download_queue() {
                        //                 Ok(res) =>{
                        //                     info!("{} added to donwload queue", episode.title);
                        //                 },
                        //                 Err(e) =>{
                        //                     error!("{}", e);
                        //                 }
                        //             }
                        //         }
                        //     },
                        //     Err(e) =>{
                        //         error!("{}", e);
                        //     }
                        // }
                    },
                    Err(e) => {
                        error!("{}", e);
                        break
                    }
                }
            }
        },
        Err(e) => {
            error!("{}", e)
        }

    }
}
fn display_pods_to_choose_episodes_archive(pods: &Vec<Podcast>) -> Result<i16, Error> {
    let screen = Screen::new();
    let pods_len = pods.len(); 
    let result: i16 = -1;
    if pods.len() == 0 {
        error_message(format!("No Podcasts to display.").as_str());
        return Ok(result)
    }
    let display_size: u16 = screen.row_size -1;
    let mut pages: u16 = 0;
    let mut page_iter = 0;
    let mut row_iter; // = 0;
    if (pods_len as u16).rem_euclid(display_size) > 0 {
        pages += 1;
        pages += (pods_len as u16)/(display_size);
    }
    
    loop {
        println!("\x1B[2J\x1B[1;1H");
        info!("Display pods");
        let start = page_iter*display_size;
        let end; // = 0;

        if ((page_iter+1)*display_size)-1 < (pods_len as u16) - 1 {
            end = ((page_iter+1)*display_size)-1;
        } else {
            end = (pods_len as u16) - 1;
        }

        row_iter = start;
        
        let epi_temp: Episode = Episode::new();
        while row_iter <= end {
            match epi_temp.count_episodes_archive(pods[row_iter as usize].id) {
                Ok(count) => {
                    println!("{}. {} - {}", (row_iter + 1), pods[row_iter as usize].name, count);
                    row_iter += 1;
                },
                Err(e) => {
                    error!("{}",e)
                }

            }
        }
        let mut line = String::new();
        print!("Choice: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line);
        match line.trim_end_matches('\n') {
            "q" => break Ok(result),
            "n" => {
                if page_iter < (pages -1) {
                    page_iter += 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            "p" => {
                if page_iter > 0 {
                    page_iter -= 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            _ => {
                info!("display_pods not q, n, or p");
                match line.trim().parse::<i16>() {
                    Ok(line_parsed) => {
                        let episode: Episode = Episode::new();
                        match episode.read_all_episodes_by_podcast_id(pods[(line_parsed as usize)-1].id, None) {
                            Ok(episodes) =>{
                                match display_episodes(&episodes) {
                                    Ok(chosen_epis) => {
                                        info!("{:?}", chosen_epis);
                                        add_episodes_to_download_queue(chosen_epis);

                                    },
                                    Err(e) => {
                                        error!("{}", e)
                                    }
                                }
                            },
                            Err(e) => {
                                error!("{}", e)
                            }
                        }
                    },
                    Err(_) => {
                        error_message(format!("{} is not a valid number.", line.trim()).as_str())
                    }

                }
            }
        }
    }
}
fn display_pods_to_choose_episodes_archive2(pods: &Vec<Podcast>) -> Result<Podcast, Error> {
    let screen = Screen::new();
    let pods_len = pods.len(); 
    let result: i16 = -1;
    if pods.len() == 0 {
        error_message(format!("No Podcasts to display.").as_str());
        return Err(Error::QueryReturnedNoRows)
    }
    let display_size: u16 = screen.row_size -1;
    let mut pages: u16 = 0;
    let mut page_iter = 0;
    let mut row_iter; // = 0;
    if (pods_len as u16).rem_euclid(display_size) > 0 {
        pages += 1;
        pages += (pods_len as u16)/(display_size);
    }
    
    loop {
        println!("\x1B[2J\x1B[1;1H");
        info!("Display pods");
        let start = page_iter*display_size;
        let end; // = 0;

        if ((page_iter+1)*display_size)-1 < (pods_len as u16) - 1 {
            end = ((page_iter+1)*display_size)-1;
        } else {
            end = (pods_len as u16) - 1;
        }

        row_iter = start;
        
        let epi_temp: Episode = Episode::new();
        while row_iter <= end {
            match epi_temp.count_episodes_archive(pods[row_iter as usize].id) {
                Ok(count) => {
                    println!("{}. {} - {}", (row_iter + 1), pods[row_iter as usize].name, count);
                    row_iter += 1;
                },
                Err(e) => {
                    error!("{}",e)
                }

            }
        }
        let mut line = String::new();
        print!("Choice: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line);
        match line.trim_end_matches('\n') {
            "q" => break return Err(Error::QueryReturnedNoRows),
            "n" => {
                if page_iter < (pages -1) {
                    page_iter += 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            "p" => {
                if page_iter > 0 {
                    page_iter -= 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            _ => {
                info!("display_pods not q, n, or p");
                match line.trim().parse::<i16>() {
                    Ok(line_parsed) => {
                        return Ok(pods[(line_parsed as usize)-1].clone())
                        // let episode: Episode = Episode::new();
                        // match episode.read_all_episodes_by_podcast_id(pods[(line_parsed as usize)-1].id) {
                        //     Ok(episodes) =>{
                        //         match display_episodes(&episodes) {
                        //             Ok(chosen_epis) => {
                        //                 info!("{:?}", chosen_epis);
                        //                 add_episodes_to_download_queue(chosen_epis);

                        //             },
                        //             Err(e) => {
                        //                 error!("{}", e)
                        //             }
                        //         }
                        //     },
                        //     Err(e) => {
                        //         error!("{}", e)
                        //     }
                        // }
                    },
                    Err(_) => {
                        error_message(format!("{} is not a valid number.", line.trim()).as_str());
                        return Err(Error::QueryReturnedNoRows)
                    }

                }
            }
        }
    }
}




fn display_episodes(epis: &Vec<Episode>) -> Result<Vec<Episode>, Error> {
    info!("{}", epis.len());
    let screen = Screen::new();
    let epis_len = epis.len(); 
    let mut results: Vec<Episode> = Vec::new();
    if epis.len() == 0 {
        error_message(format!("No Episodes in download queue.").as_str());
        return Ok(results)
    }
    let display_size: u16 = screen.row_size -1;
    let mut pages: u16 = 0;
    let mut page_iter = 0;
    let mut row_iter; // = 0;
    if (epis_len as u16).rem_euclid(display_size) > 0 {
        pages += 1;
        pages += (epis_len as u16)/(display_size);
    }
    
    loop {
        println!("\x1B[2J\x1B[1;1H");
        info!("Display epis");
        let start = page_iter*display_size;
        let end; // = 0;

        if ((page_iter+1)*display_size)-1 < (epis_len as u16) - 1 {
            end = ((page_iter+1)*display_size)-1;
        } else {
            end = (epis_len as u16) - 1;
        }

        row_iter = start;
        while row_iter <= end {
            match epis[row_iter as usize].mark_viewed() {
                Ok(marked) =>{
                    println!("{}. {}", (row_iter + 1), epis[row_iter as usize].title);
                    row_iter += 1;
                },
                Err(e) => {
                    error!("{}", e)
                }
            }
        }
        let mut line = String::new();
        print!("Choice: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line);
        match line.trim_end_matches('\n') {
            "q" => break Ok(results),
            "n" => {
                if page_iter < (pages -1) {
                    page_iter += 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            "p" => {
                if page_iter > 0 {
                    page_iter -= 1;
                } else {
                    // do nothing bitches
                }
                continue
            },
            _ => {
                info!("display_epis not q, n, or p");
                let all: Vec<&str> = line.trim_end_matches('\n').split(",").collect();
                for each in all {
                    if each.contains("-") {
                        info!("has a dash" );
                        let dash: Vec<&str> = each.split("-").collect();
                        if dash.len() > 2 {
                            error_message(format!("{each} is formatted incorrectly").as_str());
                            break
                        } else {
                            match dash[0].parse::<u16>() {
                                Ok(val) => {
                                    match (dash[1]).parse::<u16>() {
                                        Ok(val2) =>{
                                            if val >= (start + 1) && val2 <= (end + 1) {
                                                for v in val..=val2 {
                                                    info!("{}", v);
                                                    results.push(epis[(v as usize)-1].clone());
                                                }
                                            }
                                        },
                                        Err(_) => {
                                            let temp = dash[1];
                                            error_message(format!("{temp} is not a valid nubmer.").as_str());
                                        }
                                    }
                                },
                                Err(_) => {
                                    let temp = dash[0];
                                    error_message(format!("{temp} is not a valid nubmer.").as_str());
                                }
                            }
                        }
                    } else  {
                        match each.parse::<u16>() {
                            Ok(val) => {
                                results.push(epis[(val as usize)-1].clone());
                            },
                            Err(_) => {
                                error_message(format!("{} is not a valid number.", each).as_str())
                            }
                        }
                    }

                }
            }
        }

    }
}
fn start_downloads() {
    match Episode::read_all_in_download_queue() {
        Ok(episodes) => {
            for episode in episodes {
                info!("Attempting to download {}", episode.title);
                match episode.get_podcast_download_location() {
                    Ok(download) =>{
                        let path = Path::new(&episode.url);
                        let mut filename: String = "".to_string(); 
                        filename.push_str(&download);
                        let path_temp: String = path.file_name().unwrap().to_str().unwrap().to_string();
                        filename.push_str(&path_temp);

                        let client = reqwest::Client::new();
                        let rt = tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .unwrap();

                        
                        rt.block_on(async { match download_file(&client, &episode.url, &filename).await {
                            Ok(res) =>{
                                match episode.mark_downloaded() {
                                    Ok(downloaded) => {
                                        info!("{} marked as downloaded", episode.title);
                                    },
                                    Err(e) => {
                                        error!("{}",e);
                                    }
                                }
                            },
                            Err(e) =>{
                                error!("{}",e)
                            }
                        } })
                    },
                    Err(e) => {
                        error!("{}",e)
                    }
                }
            }
        },
        Err(e) => {
            error!("{}",e)
        }
    }
}
async fn download_file(client: &Client, url: &str, path: &str) -> Result<(), String> {
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
    
    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    // pb.set_message(&format!("Downloading {}", url));

    // download chunks
    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        file.write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(&format!("Downloaded {} to {}", url, path));
    return Ok(());
}

async fn download_file2(epi: &Episode, path: &str) -> Result<(), ReqError>  {
    // let mut file = File::create(path).await?;
    // println!("Downloading {}...", epi.url.clone());

    // let mut stream = reqwest::get(epi.url.clone())
    //     .await?
    //     .bytes_stream();

    // while let Some(chunk_result) = stream.next().await {
    //     let chunk = chunk_result?;
    //     file.write_all(&chunk).await?;
    // }

    // file.flush().await?;

    // println!("Downloaded {}", url);
    Ok(())
}
// async fn download_file3(url: &str, path: &str) -> Result<(), Error>  {

//     match File::create(path).await {
//         Ok(file) => {
//             println!("Downloading {}...", url);

//             let mut stream = reqwest::get(url)
//                 .await?
//                 .bytes_stream();
        
//             while let Some(chunk_result) = stream.next().await {
//                 let chunk = chunk_result?;
//                 file.write_all(&chunk).await?;
//             }
        
//             file.flush().await?;
        
//             println!("Downloaded {}", url);
//             Ok(())
//         },
//         Err(e) =>{
//             error!("{}",e)
//         }
//     }
//     // let mut file = File::create(path).await?;
//     // println!("Downloading {}...", url);

//     // let mut stream = reqwest::get(url)
//     //     .await?
//     //     .bytes_stream();

//     // while let Some(chunk_result) = stream.next().await {
//     //     let chunk = chunk_result?;
//     //     file.write_all(&chunk).await?;
//     // }

//     // file.flush().await?;

//     // println!("Downloaded {}", url);
//     // Ok(())
// }
// fn dc(cats: &Result<Vec<Category>>){

// }
// fn display_cats(cats: &Result<Vec<Category>>) -> Result<std::string::String> {
// fn display_cats_for_new_podcast(cats: &Vec<Category>) -> Result<String, Error> {
//         let cats_len: i32 = cats.len() as i32; 
//         for (i, ct) in cats.iter().enumerate() {
//             println!("{}. {}",(i+1),ct.name);
//         }
//         let mut line = String::new();
//         print!("Choice: ");
//         io::stdout().flush().unwrap();
//         std::io::stdin().read_line(&mut line).unwrap();
//         match line.trim().parse::<i32>() {
//             Ok(val) => {
//                 if val <= cats_len  && val > 0 {
//                     return Ok(val.to_string())
//                 } else {
//                     return Err(Error::InvalidQuery:rusqlite::Error)
//                 }
//             }
//             Err(e) => {
//                 match line.trim() {
//                     "q" | "n" => Ok("".to_string()),
//                     _err => Ok("".to_string())
//                 }
//             }
//         }
//     }
fn display_cats2(cats: &Result<Vec<Category>, Error>) -> Result<std::string::String, Error> {
    let cats_len = cats.as_ref().unwrap().len(); 
    let result: String;
    loop {      
        println!("\x1B[2J\x1B[1;1H");
        for (i, ct) in cats.as_ref().unwrap().iter().enumerate() {
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
                    "q" => return Err((Error::InvalidColumnName("".to_string()))),
                    _err => { return Err((Error::InvalidColumnName(_err.to_string())))}
                }
            }
        }
    }

}
fn display_cats3(cats: Vec<Category>) -> Result<std::string::String, Error> {
    let cats_len = cats.len(); 
    loop {      
        println!("\x1B[2J\x1B[1;1H");
        for (i, ct) in cats.iter().enumerate() {
            println!("{}. {}",(i+1),ct.name);
        }
        let mut line = String::new();
        print!("Choose number or press enter for all: ");
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
                    "" => return Ok("".to_string()),
                    "q" => return Err((Error::InvalidColumnName("".to_string()))),
                    _err => {}
                }
            }
        }
    }
}
fn display_cats4(cats: Vec<Category>) -> Result<Category, Error> {
    let cats_len = cats.len(); 
    loop {      
        println!("\x1B[2J\x1B[1;1H");
        for (i, ct) in cats.iter().enumerate() {
            println!("{}. {}",(i+1),ct.name);
        }
        let mut line = String::new();
        print!("Choose number or press enter for all: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        match line.trim().parse::<usize>() {
            Ok(val) => {
                if val <= cats_len  && val > 0 {
                    return Ok(cats[val -1].clone())
                }
            }
            Err(_) => {
                match line.trim() {
                    "" => return Err((Error::InvalidColumnName("".to_string()))),
                    // "" => return Ok("".to_string()),
                    "q" => return Err((Error::InvalidColumnName("".to_string()))),
                    _err => return  Err((Error::InvalidColumnName("".to_string())))
                    // _err => {}
                }
            }
        }
    }
}
fn quit() {
    info!("quitting");
    process::exit(1);
}
