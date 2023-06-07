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

use home;

use std::fs;
use std::io::{self, Read, Write, BufRead};
use std::path::PathBuf;

use api::api::AppleSearch;
use api::retrieve::Retreive;
use data::podcast::Podcast;

use data::category::Category;
use rusqlite::{Connection, NO_PARAMS};

use menu::menu_entry::MenuEntry;
use menu::category_menu::CategoryMenu;
use menu::screen::Screen;
use menu::simple_menu::SimpleMenu;

use std::{thread, time};

use std::process;

use log::{info, warn, error, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

// use rustyline::{Editor, Result as rlResult};
use rustyline::{DefaultEditor, Result as rlResult};
use rusqlite::{Error};
use rustyline::error::ReadlineError;

use std::collections::HashSet;

fn main() {
    // systemd_journal_logger::init().unwrap();
    // log::set_max_level(LevelFilter::Info);
    
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    info!("logging started");
    let screen = Screen::new();
    // let main_menu = create_main_menu();
    // let cat_name = CategoryMenu{};

    // let category_menu = CategoryMenu::new();
    let category_menu = CategoryMenu{
        screen: Screen::new()
    };

    
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
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("Start downloads"),
        reference: (entries.len() + 1).to_string(),
        f: trial,
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
        f: trial,
        show: true
    });
    entries.push(MenuEntry {
        description: String::from("update all podcasts"),
        reference: (entries.len() + 1).to_string(),
        f: update_podcast_download_info,
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




    info!("just before config call");
    let config = config::config::Config::new();
    info!("Just after config call");
    // let simple_menu = SimpleMenu::new(screen, main_menu);
    let simple_menu = SimpleMenu::new(Screen::new(), entries);
    // let simple_menu = SimpleMenu::new(screen, entries);
    // let simple_menu = SimpleMenu::new(screen, create_main_menu());
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
    // let db = data::data::DB::new(config::config::Config::new());
    // let conn = db.connect_to_database();
    let mut cat = Category::new();
    
    cat.name = enter_category_info2("Enter Category name".to_string());
    let temp = cat.create_exisitng();
    // let temp = cat.create_category(conn);
    info!("temp");
    info!("{}",temp.unwrap());
    // println!("Hello , {}", line);

    thread::sleep(time::Duration::from_secs(2));
}
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
fn enter_category_info2(message: String) -> String {
    println!("\x1B[2J\x1B[1;1H");
    info!("create_new_cateory");
    let mut line = String::new();
    print!("{}: ", message);
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    return line;
}
fn enter_category_info3(message: &str, default: &str) -> Result<std::string::String, ReadlineError> {
    // let mut rl = Editor::new()?;
    let mut rl = DefaultEditor::new()?;
    // let mut rl: Editor<H> = rustyline::Editor::new()?;
    // const DEFAULT_USERNAME: &str = "admin";
    let mut input: String = "".to_string();
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
    // println!("Your selected username: {input}");
    // Ok(input)

}
fn delete_category()  {
    let db = data::data::DB::new(config::config::Config::new());
    let conn = db.connect_to_database();
    let mut cat = Category::new();
    let cats = cat.read_categories(&conn);
    let result = display_cats2(&cats);
    match result {
        Ok(index) => {
            match index.trim().parse::<usize>() {
                Ok(idx) => {
                    cat.id = cats.unwrap()[idx-1].id;
                    let res = cat.delete_category(conn);
                    info!("{}", res.unwrap());
                },
                Err(e) => error!("delete_category() error is {}", e)
                // Err(e) => {e.to_string()}
                // Err(e) => {e.to_string()};
            }
        },
        // Err(e) => e.to_string(),
        Err(e) => error!("delete_category() error is {}", e)
    }

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
fn edit_category() {
    let db = data::data::DB::new(config::config::Config::new());
    let conn = db.connect_to_database();
    let mut cat = Category::new();
    let cats = cat.read_categories(&conn);
    // let result = dc(&cats);
    let result = display_cats2(&cats);

    match result {
        Ok(index) => {
            info!("index :{:?}", index);
            match index.trim().parse::<usize>() {
                Ok(idx) => {
                    // info!("ok");
                    // info!("{}",idx);
                    // info!("{}", cats.unwrap()[idx-1].id);
                    // info!("{}",cat.id);
                    cat.id = cats.unwrap()[idx-1].id;
                    let res2 = cat.read_category_by_id(&conn, idx);
                    match res2 {
                        Ok(temp_cat) =>{
                            info!("temp_cat.id:{:?}", temp_cat.id);
                            let res_name = enter_category_info3("Existing Category name: ",&temp_cat.name).unwrap();
                            if res_name.len() > 0 {
                                cat.name = res_name.to_string();
                                let update = cat.update_category(&conn);
                                match update {
                                    Ok(_) => {},
                                    // Err(_) => { error_message("Could not update the Category.")}
                                    Err(_) => { error_message("Could not update the Category.")}

                                    // match index.trim() {
                                    //     "q" => {},
                                    //     _err => error_message(&format!("edit_category() error is {}", e))
                                    // }
                                }
                            } else {
                                // nothing was entered - don't update
                            }
                        },
                        Err(e) => error_message(&format!("edit_category() error is {}", e))
                    }

                },
                Err(e) =>  {
                    match index.as_str() {
                        "q" => {},
                        _ => error_message(&format!("2edit_category() error is {:?}", index))

                    }
                }
            }
            // info!("index is {}", index.trim().parse::<i32>().unwrap() -1)
        },
        Err(e) => error_message(&format!("edit_category() error is {}", e))
    }
    // cats[result.trim().parse::<i32>().unwrap()-1].name = enter_category_info();
}
fn search() {
    let terms = enter_search_terms();
    let search = AppleSearch::new("https://itunes.apple.com".to_string(),terms.to_string(),50);
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
    // let choices = display_pods(&results);
    // info!("search {:?}", results);
    // info!("search choices {:?}", choices);
}

fn delete_podcast() {
    let temp_pod: Podcast = Podcast::new();
    match temp_pod.read_all_podcasts() {
        Ok(mut tmp ) => {
            match display_pods(&tmp) {
                Ok(chosen) => {
                    info!("delete_podcast chosen.len(): {}", chosen.len());
                    if chosen.len() > 0 {
                        let mut v: Vec<&u16> = chosen.iter().collect();
                        v.sort();
                        info!("delete_podcast v: {:?}", v);
                        for each in v {
                            info!("each: {}", (*each as usize)-1);
                            info!("Podcast returned {:?}",tmp[(*each as usize)-1]);
                            match tmp[(*each as usize)-1].delete_existing() {
                                Ok(tmp) => {
                                    info!("Ok tmp: {:?}", tmp);
                                },
                                Err(e) => {
                                    error!("{}", e);
                                }
                            }
                            
                        }
                    } else {
                        // quit without choosing
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
fn edit_podcast() {
    println!("\x1B[2J\x1B[1;1H");
    let mut pod: Podcast = Podcast::new();
    match pod.read_all_podcasts() {
        Ok(mut res) =>{
            match display_pods(&res) {   
                Ok(chosen) =>{
                    info!("Ok(chosen");
                    let mut v: Vec<&u16> = chosen.iter().collect();
                    v.sort();
                    for each in v {
                        info!("Podcast returned {:?}",res[(*each as usize)-1]);
                        edit_podcast_details(res[(*each as usize)-1].clone());
                        // match res[*each as usize].save_existing() {
                        //     Ok(tmp) => {
                        //         info!("Ok tmp: {}", tmp);
                        //     },
                        //     Err(e) => {
                        //         error!("{}", e);
                        //     }
                        // }
                        
                    }
                },
                Err(_) => {
                    error_message(format!("Had an issue returing the podcasts").as_str())
                }
            }
        },
        Err(_) => {
            error_message(format!("Had an issue returing the podcasts").as_str())
        }
    }

}

fn edit_podcast_details(mut pod: Podcast) {
    match enter_category_info3("Podcast name: ", &pod.name) {
        Ok(name) => {
            pod.name = name;
            info!("New Podcast name: {}", pod.name);
            match enter_category_info3("Podcast URL: ", &pod.url) {
                Ok(url) => {
                    pod.url = url;
                    info!("New Podcast URL: {}", pod.url);
                    match enter_category_info3("Default Audio Save location : ", &pod.audio) {
                        Ok(audio) =>{
                            pod.audio = audio;
                            info!("New Podcast default audio location: {}", pod.audio);
                            match enter_category_info3("Default Audio Save location : ", &pod.video) {
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

fn create_podcast() {
    println!("\x1B[2J\x1B[1;1H");
    let mut pod: Podcast = Podcast::new();
    edit_podcast_details(pod);

}

fn display_pods(pods: &Vec<Podcast>) -> Result<HashSet<u16>, Error> {
    let screen = Screen::new();
    let pods_len = pods.len(); 
    let mut results: HashSet<u16> = HashSet::new();
    if pods.len() == 0 {
        error_message(format!("No Podcasts to display.").as_str());
        return Ok(results)
    }
    // info!("pods[0]: {:?}", pods[0]);
    let display_size: u16 = screen.row_size -1;
    // info!("display_size:{}", display_size);
    let mut pages: u16 = 0;
    let mut page_iter = 0;
    let mut row_iter = 0;
    if (pods_len as u16).rem_euclid(display_size) > 0 {
        pages += 1;
        pages += (pods_len as u16)/(display_size);
    }
    
    loop {
        println!("\x1B[2J\x1B[1;1H");
        info!("Display pods");
        let start = page_iter*display_size;
        let mut end = 0;

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
                // info!("next {}", page_iter);
                continue
            },
            "p" => {
                if page_iter > 0 {
                    page_iter -= 1;
                } else {
                    // do nothing bitches
                }
                // info!("prev {}", page_iter);
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
                // info!("{:?}", results);
            }
        }
        // match line.as_str() {
        //     "q" => info!("quit"),
        //     // "q" => break Ok(results),
        //     _ => info!("{}",line)
        // }
        // match line.trim().parse::<i32>() {
        //     Ok(val) => {
        //         if val <= cats_len as i32  && val > 0 {
        //             return Ok(results);
        //         }
        //     }
        //     Err(_) => {
        //         match line.trim() {
        //             "q" => break Ok(results),
        //             // "n" => break,
        //             // "q" | "n" => return Ok(String::from(line.trim())),
        //             _err => {}
        //         }
        //     }
        // }
    }
}

fn update_podcast_download_info() {
    let retrieve = Retreive::new();
    
    let temp_pod: Podcast = Podcast::new();
    match temp_pod.read_all_podcasts() {
        Ok(pods)=>{
            for pod in pods {
                info!("{}, {}", pod.url, pod.id);
                match retrieve.retreive_episodes(pod.url, pod.id as i16) {
                    Ok(mut episodes) =>{
                        info!("{:?}", episodes.len());
                        for mut episode in episodes {
                            info!("{}", episode.title);
                            match episode.save_existing() {
                                Ok(res) => {
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
                    "q" | "n" => break Ok("q".to_string()),
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

// fn create_main_menu() -> Vec<MenuEntry> {
//     let mut entries: Vec<MenuEntry> = Vec::new();

//     entries.push(MenuEntry {
//         description: String::from("Add new category"),
//         reference: (entries.len() + 1).to_string(),
//         f: create_new_category,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("Edit category"),
//         reference: (entries.len() + 1).to_string(),
//         f: edit_category,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("Delete category"),
//         reference: (entries.len() + 1).to_string(),
//         f: delete_category,
//         show: true
//     });
//         entries.push(MenuEntry {
//         description: String::from("Add new podcast"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("Edit podcast"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("Delete podcast"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("Choose episodes to download"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("Start downloads"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("Search for podcasts"),
//         reference: (entries.len() + 1).to_string(),
//         f: search,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("delete from download queue"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("update all podcasts"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });
//     entries.push(MenuEntry {
//         description: String::from("archive"),
//         reference: (entries.len() + 1).to_string(),
//         f: trial,
//         show: true
//     });

//     entries.push(MenuEntry {
//         description: String::from("quit"),
//         reference: "q".to_owned(),
//         f: quit,
//         show: false
//     });

//     entries
// }
