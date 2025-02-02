use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Error, Result};
use log::{error,info};

use super::super::config::config::Config;
use super::super::menu::screen::Screen;
use super::podcast::Podcast;
use super::category::Category;
use super::data::DB;
use std::io::{self, Write};
use std::fs::File;
use futures_util::StreamExt;

use super::super::utilities::utilities::error_message;

use std::clone::Clone;
use std::path::Path;

use std::fmt;

#[derive(Debug, Clone)]
struct ReqwestError;
impl fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[derive(Debug)]
pub enum MyErrors {
    SomeError,
    StdError
}
impl fmt::Display for MyErrors {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        match self {
            MyErrors::SomeError => write!(f, "MyErrors"),
            MyErrors::StdError =>  write!(f, "stdnard error")
        }
    }

}


#[derive(Debug)]
pub struct Count {
    pub result: usize
}

#[derive(Debug)]
pub struct Download {
    pub result: String
}

#[derive(Debug)]
pub struct Episode {
    pub id: i64,
    pub title: String,
    pub published: DateTime<Utc>,
    pub summary: String,
    pub length: i32,
    pub audio: String,
    pub url: String,
    pub downloaded: i8,
    pub viewed: i8,
    pub podcast_id: i16,
    pub queue: i8
}




impl Episode {
    pub fn new() -> Episode {
        Episode {
            id: 0,
            title: String::from("nada"),
            published: Utc::now(),
            summary: String::from("nada"),
            length: 3600,
            audio: String::from("audio/mpeg"),
            url: String::from("nada"),
            downloaded: 0,
            viewed: 0,
            podcast_id: 1,
            queue: 0
        }
    }
    
    fn db_execute(&self, statement: &str) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        match conn.execute(statement, params![]) {
            Ok(result) => {
                Ok(result)
            },
            Err(e) => {
                return Err(e)
            }
        }
    }
    pub fn remove_from_download_queue(&self) -> Result<usize, Error> {
        self.change_download_status(0)
    }
    pub fn add_to_download_queue(&self) -> Result<usize, Error> {
        self.change_download_status(1)
    }
    fn change_download_status(&self, queue: u8) -> Result<usize, Error> {
        let stmt: String = format!("UPDATE episodes SET queue={} where episode_id={}", queue, self.id);
        self.db_execute(&(stmt.clone()))
    }
    pub fn mark_viewed(&self) -> Result<usize, Error> {
        let stmt: String = format!("UPDATE episodes SET viewed=1 where episode_id={}", self.id);
        self.db_execute(&(stmt.clone()))
    }
    pub fn mark_downloaded(&self) -> Result<usize, Error> {
        let stmt: String = format!("UPDATE episodes SET queue=0 AND downloaded=1 where episode_id={}", self.id);
        self.db_execute(&(stmt.clone()))
    }
    // pub fn count_episodes(&self, cat_id: i64) -> Result<usize, Error> {
    //     let stmt: String = format!("SELECT COUNT(viewed) from episodes where podcast_id={} and viewed=0;", cat_id);
    //     self.count_helper(&(stmt.clone()))
    // }
    pub fn count_episodes_archive(&self, cat_id: i64) -> Result<usize, Error> {
        let stmt: String = format!("SELECT COUNT(viewed) from episodes where podcast_id={};", cat_id);
        self.count_helper(&(stmt.clone()))
    }
    fn count_helper(&self, statement: &str) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        
        let mut stmt = conn.prepare(statement)?;
        match stmt.query_map([], |row| {
            Ok(Count {
                result: row.get(0)?
            })
        }) {
            Ok(mut res) =>{
                match res.next() {
                    Some(count) =>{
                        match count {
                            Ok(result) => {
                                return Ok(result.result)
                            },
                            Err(e) => {
                                return Err(e)
                            }
                        }
                    },
                    None =>{
                        return Err(Error::QueryReturnedNoRows)
                    }
                }

            },
            Err(e) =>{
                return Err(e)
            }
        };

    }
    pub fn save_existing(&mut self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let result = conn.execute(
            "INSERT INTO episodes (title, published, summary, length, audio, url, downloaded, podcast_id,viewed, queue) VALUES (?1, ?2, ?3,?4, ?5, ?6, ?7, ?8, ?9, ?10)", 
            params![self.title, self.published, self.summary,self.length, self.audio, self.url, self.downloaded, self.podcast_id,0, 0]
        )?;
        self.id = conn.last_insert_rowid();
        Ok(result)
    }
    pub fn read_all_episodes_by_podcast_id(&self, pod_id: i64, viewed: Option<i64>) ->Result<Vec<Episode>, Error>  {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let mut stmt;

        match viewed {
            Some(_) =>{
                stmt = conn.prepare("SELECT * FROM episodes where podcast_id=(?) ORDER BY title ASC;")?
            },
            None =>{
                stmt = conn.prepare("SELECT * FROM episodes where podcast_id=(?) AND viewed=0 ORDER BY title ASC;")?
            }
        }
        let epi_iter = stmt.query_map([pod_id], |row| {
            Ok(Episode {
                id: row.get(0)?,
                title: row.get(1)?,
                published: row.get(2)?,
                summary: row.get(3)?,
                length: row.get(4)?,
                audio: row.get(5)?,
                url: row.get(6)?,
                viewed: row.get(7)?,
                downloaded: row.get(8)?,
                podcast_id: row.get(9)?,
                queue: row.get(10)?
            })
        })?;
        let mut results: Vec<Episode> = Vec::new();
        for episode in epi_iter {
            results.push(episode.unwrap());
        }
        Ok(results)
    }
    pub fn get_podcast(&self) -> Result<Podcast, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let statement: String = format!("Select * from podcasts where podcast_id={};", self.podcast_id);

        let _x = match conn.prepare(&statement.clone()) {
            Ok(mut prepared_statement) => {
                info!("test");
                match prepared_statement.query_map([],|row| {
                    Ok(Podcast {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        url: row.get(2)?,
                        audio: row.get(3)?,
                        video: row.get(4)?,
                        category_id: row.get(5)?,
                        collection_id: row.get(6)?,
                        notviewed: row.get(7)?
                    })
                }) {
                    Ok(mut pod_itr) =>{
                        match pod_itr.next() {
                            Some(podcast) => {
                                return podcast
                            },
                            None => {
                                return Err(Error::QueryReturnedNoRows)
                            }
                        }

                    },
                    Err(e) => {
                        return Err(e)
                    }
                }
            },
            Err(e) => {
                return Err(e)
            }
        };

    }
    pub fn read_all_in_download_queue() -> Result<Vec<Episode>, Error> {
        let mut results: Vec<Episode> = Vec::new();
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let mut _stmt = match conn.prepare("SELECT * FROM episodes where queue=1;") {
            Ok(mut prepared_statement) =>{
                match prepared_statement.query_map([], |row| {
                    Ok(Episode {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        published: row.get(2)?,
                        summary: row.get(3)?,
                        length: row.get(4)?,
                        audio: row.get(5)?,
                        url: row.get(6)?,
                        downloaded: row.get(7)?,
                        podcast_id: row.get(8)?,
                        viewed: row.get(9)?,
                        queue: row.get(10)?
                    })
                }) {
                    Ok(epi_iter) =>{
                        for episode in epi_iter {
                            results.push(episode.unwrap());
                        };
                        return Ok(results)
                    },
                    Err(e) => {
                        return Err(e)
                    }
                }
            },
            Err(e) =>{
                return Err(e)
            }
        };
    }
    // fn display_pods_single_result2_epi(pods: &Vec<Podcast>, count: Option<String>) -> Result<Podcast, Error> {
    //     let screen = Screen::new();
    //     let pods_len = pods.len(); 
    //     if pods.len() == 0 {
    //         error_message(format!("No Podcasts to display.").as_str());
    //         return Err(Error::InvalidColumnName("".to_string()))
    //     }
    //     let display_size: u16 = screen.row_size -1;
    //     let mut pages: u16 = 0;
    //     let mut page_iter = 0;
    //     let mut row_iter; // = 0;
    //     if (pods_len as u16).rem_euclid(display_size) > 0 {
    //         pages += 1;
    //         pages += (pods_len as u16)/(display_size);
    //     }
        
    //     loop {
    //         println!("\x1B[2J\x1B[1;1H");
    //         // info!("Display pods");
    //         let start = page_iter*display_size;
    //         let end; // = 0;
    
    //         if ((page_iter+1)*display_size)-1 < (pods_len as u16) - 1 {
    //             end = ((page_iter+1)*display_size)-1;
    //         } else {
    //             end = (pods_len as u16) - 1;
    //         }
    
    //         row_iter = start;
    //         while row_iter <= end {
    //             match count {
    //                 Some(_) =>{
    //                     // let cnt: usize = Episode::new().count_episodes(pods[row_iter as usize].id);
    //                     match Episode::new().count_episodes(pods[row_iter as usize].id) {
    //                         Ok(cnt) =>{
    //                             println!("number {}. {} - {}", (row_iter + 1), pods[row_iter as usize].name, cnt);
    //                         },
    //                         Err(_) =>{
    //                             println!("number {}. {}", (row_iter + 1), pods[row_iter as usize].name);
    //                         }
    //                     }
                       
    //                 },
    //                 None =>{
    //                     println!("number {}. {}", (row_iter + 1), pods[row_iter as usize].name);
    
    //                 }
    //             }
    //             row_iter += 1;
    //         }
    //         let mut line = String::new();
    //         print!("Choice: ");
    //         io::stdout().flush().unwrap();
    //         match std::io::stdin().read_line(&mut line) {
    //             Ok(_) => {
    //                 match line.trim_end_matches('\n') {
    //                     "q" => return Err(Error::InvalidColumnName("".to_string())),
    //                     "n" => {
    //                         if page_iter < (pages -1) {
    //                             page_iter += 1;
    //                         } else {
    //                             // do nothing bitches
    //                         }
    //                         continue
    //                     },
    //                     "p" => {
    //                         if page_iter > 0 {
    //                             page_iter -= 1;
    //                         } else {
    //                             // do nothing bitches
    //                         }
    //                         continue
    //                     },
    //                     _ => {
    //                         // info!("display_pods not q, n, or p");
    //                         match line.trim_end_matches('\n').parse::<u16>() {
    //                             Ok(val) => {
    //                                 return Ok(pods[(val as usize)-1].clone())
    //                             },
    //                             Err(_) => {
    //                                 error_message(format!("{} is not a valid number.", line.trim()).as_str())
    //                             }
    //                         }
    //                     }
    //                 }
    //             },
    //             Err(e) => {
    //                 error!("{}",e);
    //             }
    //         }
    
    //     }
    // }
    pub fn add_episodes_to_download_queue(episode_list: Vec<Episode>) {
        for episode in episode_list{
            match  episode.add_to_download_queue() {
                Ok(_) =>{
                    info!("{} added to download queue", episode.title);
                },
                Err(e) =>{
                    error!("{}", e);
                    error_message(format!("Could not add episode {} to download queue.", episode.title).as_str());
                }
                
            }
        }

    }

    pub fn choose_episodes_epi() {
        match Category::display() {
            Ok( category) =>{
                match Podcast::display(Some(category)) {
                    Ok(chosen_pod) => {
                        match Episode::new().read_all_episodes_by_podcast_id(i64::from(chosen_pod), None) {
                            Ok(all_episodes) =>{
                                match Episode::display_episodes_epi(&all_episodes) {
                                    Ok(chosen_episodes) =>{
                                       Episode::add_episodes_to_download_queue(chosen_episodes)
                                    },
                                    Err(e) =>{
                                        error!("{}", e);
                                    }
                                }

                            },
                            Err(e) =>{
                                error!("{}",e);
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
        };

        // match Category::new().read_all_categories() {
        //     Ok(cats) => {
        //         match Category::display_cats_cat(cats) {
        //             Ok(c) => {
        //                 match Podcast::read_all_podcasts2(Some(c.id.to_string())) {
        //                     Ok(pods) =>{
        //                         loop {
        //                             match Episode::display_pods_single_result2_epi(&pods, Some("".to_string())) {
        //                                 Ok(chosen_pod) => {
        //                                     match Episode::new().read_all_episodes_by_podcast_id(chosen_pod.id, None) {
        //                                         Ok(all_episodes) =>{
        //                                             match Episode::display_episodes_epi(&all_episodes) {
        //                                                 Ok(chosen_episodes) =>{
        //                                                    Episode::add_episodes_to_download_queue(chosen_episodes)
        //                                                 },
        //                                                 Err(e) =>{
        //                                                     error!("{}", e);
        //                                                 }
        //                                             }
    
        //                                         },
        //                                         Err(e) =>{
        //                                             error!("{}",e);
        //                                         }
        //                                     }
        //                                 },
        //                                 Err(e) => {
        //                                     error!("{}", e);
        //                                     break
        //                                 }
        //                             }
        //                         }
        //                     },
        //                     Err(e) => {
        //                         error!("{}", e)
        //                     }
        //                 }
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
    }
    // pub fn choose_episodes_epi2() {
    //     match Category::new().read_all_categories() {
    //         Ok(cats) => {
    //             match Category::display_cats_cat(cats) {
    //                 Ok(c) => {
    //                     match Podcast::read_all_podcasts2(Some(c.id.to_string())) {
    //                         Ok(pods) =>{
    //                             loop {
    //                                 match Episode::display_pods_single_result2_epi(&pods, Some("".to_string())) {
    //                                     Ok(chosen_pod) => {
    //                                         match Episode::new().read_all_episodes_by_podcast_id(chosen_pod.id, None) {
    //                                             Ok(all_episodes) =>{
    //                                                 match Episode::display_episodes_epi(&all_episodes) {
    //                                                     Ok(chosen_episodes) =>{
    //                                                         for ch_epi in chosen_episodes {
    //                                                             match ch_epi.add_to_download_queue() {
    //                                                                 Ok(_) =>{
    //                                                                     info!("{} added to download queue", ch_epi.title);
    //                                                                 },
    //                                                                 Err(e) =>{
    //                                                                     error!("{}", e);
    //                                                                     error_message(format!("Could not add episode {} to download queue.", ch_epi.title).as_str());
    //                                                                 }
    //                                                             }
    //                                                         }
    //                                                     },
    //                                                     Err(e) =>{
    //                                                         error!("{}", e);
    //                                                     }
    //                                                 }
    
    //                                             },
    //                                             Err(e) =>{
    //                                                 error!("{}",e);
    //                                             }
    //                                         }
    //                                     },
    //                                     Err(e) => {
    //                                         error!("{}", e);
    //                                         break
    //                                     }
    //                                 }
    //                             }
    //                         },
    //                         Err(e) => {
    //                             error!("{}", e)
    //                         }
    //                     }
    //                 },
    //                 Err(e) => {
    //                     error!("{}", e)
    //                 }
    //             }
    //         },
    //         Err(e) => {
    //             error!("{}", e)
    //         }
    //     }
    // }
    pub fn display_episodes_epi(epis: &Vec<Episode>) -> Result<Vec<Episode>, Error> {
        // info!("{}", epis.len());
        let config: Config = Config::new();
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
            // info!("Display epis");
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
                    Ok(_) =>{
                        println!("number {}. {}", (row_iter + 1), epis[row_iter as usize].title);
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
            match std::io::stdin().read_line(&mut line) {
                Ok(_) =>{
                    match line.trim_end_matches('\n') {
                        "q" => break Ok(results),
                        "a" => break Ok(epis.to_vec()),
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
                            // info!("display_epis not q, n, or p");
                            let all: Vec<&str> = line.trim_end_matches('\n').split(&config.specs.separator).collect();
                            for each in all {
                                if each.contains("-") {
                                    // info!("has a dash" );
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
                                                                // info!("{}", v);
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
                },
                Err(e) =>{
                    error!("{}",e);
                }
            }
    
    
        }
    }
    fn download_helper(print: Option<usize>) {
        match Episode::read_all_in_download_queue() {
            Ok(episodes) => {
                let number_of_episodes: usize = episodes.len();
                let mut download_count: usize = 1;
                for episode in episodes {
                    info!("Attempting to download {}", episode.title); 
                    match episode.get_podcast() {
                        Ok(podcast) =>{
                            let download: String; // = "".to_string();
                            if episode.audio.contains("video") {
                                download = podcast.video.clone();
                            } else {
                                download = podcast.audio.clone();
                            }
                            match std::fs::create_dir_all(download.clone()) {
                                Ok(_) =>{
                                    let path = Path::new(&episode.url);
                                    let mut filename: String = "".to_string(); 
                                    filename.push_str(&download);
                                    let mut ext: String = "".to_string();
                                    match path.extension() {
                                        Some(what) =>{
                                            let result1:  String = what.to_str().unwrap().to_string();
                                            info!("*********{}", result1);
                                            let result2: Vec<&str> = result1.split("?").collect();
                                            info!("********{}", result2[0]);
                                            ext = result2[0].to_string();
                                        },
                                        None => {}
                                    }
                                    let path_temp: String = path.file_name().unwrap().to_str().unwrap().to_string();
                                    filename.push_str(&path_temp);
                                    info!("{}", filename);
                                    let final_file: String = format!("{}{}-{}.{}", download,podcast.name,episode.title,ext);
                                    info!("{}", final_file);
            
                                    // let client = reqwest::Client::new();
                                    let rt = tokio::runtime::Builder::new_current_thread()
                                        .enable_all()
                                        .build()
                                        .unwrap();
            
                                    
                                    rt.block_on(async { match Episode::download_file(&episode.url, &final_file, print, number_of_episodes.clone(), download_count).await {
                                        Ok(_) =>{
                                            match episode.mark_downloaded() {
                                                Ok(_) => {
                                                    info!("{} marked as downloaded", episode.title);
                                                    download_count = download_count +1;
                                                },
                                                Err(e) => {
                                                    error!("{}",e);
                                                }
                                            }
                                        },
                                        Err(e) =>{
                                            error!("{}",e)
                                        }
                                    } 
                                }
                            )
                                    
                                },
                                Err(e) => {
                                    error!("{}",e)
                                }

                            }

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
    pub fn start_downloads_epi() {
        Episode::download_helper(Some(1));
    }
    pub fn command_line_start_downloads(){
        Episode::download_helper(None);
    }
    async fn download_file(download_url: &str, full_file_path: &str, print: Option<usize>, num_episodes: usize, download_c: usize) -> Result<(), MyErrors> {
        match reqwest::get(download_url).await {
            Ok(res) =>{
                match File::create(full_file_path) {
                    Ok(mut file) =>{
                        match print {
                            Some(_) =>{
                                println!("downloading: {} {} of {}", full_file_path, download_c, num_episodes);
                            },
                            None => {}
                        }
                        let mut stream = res.bytes_stream();
                        while let Some(item) = stream.next().await {
                            match item {
                                Ok(chunk) =>{
                                    match file.write_all(&chunk) {
                                        Ok(_) =>{
                                            // nada
                                        },
                                        Err(_) =>{
                                            return Err(MyErrors::StdError)
                                        }

                                    }

                                },
                                Err(_) =>{
                                    return Err(MyErrors::SomeError)
                                }
                            }
                        }
                        Ok(())
                        
                    },
                    Err(_) =>{
                        return Err(MyErrors::SomeError)
                    }
                }
            },
            Err(_) =>{
                return Err(MyErrors::SomeError)
            }
        }

    }
    pub fn delete_episodes_from_download_queue() {
        match Self::read_all_in_download_queue() {
            Ok(episodes) =>{
                match Self::display_episodes_epi(&episodes) {
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
    fn update_helper(print: Option<usize>) {
        match Podcast::read_all_podcasts2(None) {
            Ok(pods)=>{
                for pod in pods {
                    match print {
                        Some(_) => {
                            println!("Checking episodes for {}", pod.name);
                        },
                        None => {
                            //do nothing
                        }
                    }
                    match  pod.retreive_episodes() {
                        Ok(episodes) =>{
                            for mut episode in episodes {
                                info!("Episode {} ", episode.title);
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
    pub fn command_line_update_episodes() {
        Episode::update_helper(None);
    }
    pub fn update_episodes_for_all_podcasts() {
        Episode::update_helper(Some(1));
    }


}


impl Clone for Episode {
    fn clone(&self) -> Episode {
        Episode {
            id: self.id.clone(),
            title: self.title.clone(),
            published: self.published.clone(),
            summary: self.summary.clone(),
            length: self.length.clone(),
            audio: self.audio.clone(),
            url: self.url.clone(),
            downloaded: self.downloaded.clone(),
            viewed: self.viewed.clone(),
            podcast_id: self.podcast_id.clone(),
            queue: self.queue.clone()
        }
    }
}




// #[cfg(test)]
// mod tests {
//     #[derive(Debug)]
//     struct LocalTestContext {
//         pub conn: Connection,
//     }
//     impl LocalTestContext {
//         fn new() -> LocalTestContext {
//             let _conn = TestContext::new().conn;
//             _conn
//                 .execute(
//                     "INSERT INTO categories (name) VALUES (?1)",
//                     params![String::from("News")],
//                 )
//                 .unwrap();
//             let pod = Podcast {
//                 id: 0,
//                 name: String::from("Episode 1"),
//                 url: String::from("https://somthing.com"),
//                 audio: String::from("/home/marc/audio"),
//                 video: String::from("/home/marc/video"),
//                 category_id: 1,
//             };
//             _conn
//                 .execute(
//                     "INSERT INTO podcasts (name, url, audio, video, category_id) VALUES (?1, ?2, ?3,?4, ?5)",
//                     params![pod.name, pod.url, pod.audio, pod.video, pod.category_id],
//                 )
//                 .unwrap();
//             let epi = Episode {
//                 id: 0,
//                 title: String::from("Episode 1"),
//                 published: Utc::now(),
//                 summary: String::from("Stuff about Episode 1"),
//                 length: 3600,
//                 audio: String::from("audio/mpeg"), //true
//                 url: String::from("https://something.com/epi1"),
//                 downloaded: 0, //false
//                 podcast_id: 1,
//             };
//             _conn.execute(
//                 "INSERT INTO episodes (title, published, summary, length, audio, url, downloaded, podcast_id) VALUES (?1, ?2, ?3,?4,?5,?6,?7,?8)",
//                 params![
//                     epi.title,
//                     epi.published,
//                     epi.summary,
//                     epi.length,
//                     epi.audio,
//                     epi.url,
//                     epi.downloaded,
//                     epi.podcast_id
//                     ],
//             ).unwrap();
//             LocalTestContext { conn: _conn }
//         }
//     }
//     use super::super::context::TestContext;
//     use super::*;

//     #[test]
//     fn test_create_episode() {
//         let _conn = LocalTestContext::new();
//         let mut epi = Episode {
//             id: 0,
//             title: String::from("Episode 2"),
//             published: Utc::now(),
//             summary: String::from("Stuff about Episode 2"),
//             length: 3600,
//             audio: String::from("audio/mpeg"), //true
//             url: String::from("https://something.com/epi2"),
//             downloaded: 0, //false
//             podcast_id: 1,
//         };
//         let res = epi.create_episode(_conn.conn).unwrap();
//         assert_eq!(res, 1);
//     }

//     #[test]
//     fn test_read_episode() {
//         let _conn = LocalTestContext::new();
//         let epi = Episode::new();
//         let _res = epi.read_episodes(_conn.conn).unwrap();
//         assert_eq!(_res.len(), 1);
//         assert_eq!(_res[0].id, 1);
//     }

//     #[test]
//     fn test_update_episode() {
//         let _conn = LocalTestContext::new();
//         let epi = Episode {
//             id: 1,
//             title: String::from("Episode 2-new"),
//             published: Utc::now(),
//             summary: String::from("Stuff about Episode 2"),
//             length: 3600,
//             audio: String::from("audio/mpeg"), //true
//             url: String::from("https://something.com/epi2"),
//             downloaded: 0, //false
//             podcast_id: 1,
//         };
//         let res = epi.update_episode(_conn.conn).unwrap();
//         assert_eq!(res, 1);
//     }

//     #[test]
//     fn test_delete_episode() {
//         let _conn = LocalTestContext::new();
//         let epi = Episode {
//             id: 1,
//             title: String::from("Episode 2-new"),
//             published: Utc::now(),
//             summary: String::from("Stuff about Episode 2"),
//             length: 3600,
//             audio: String::from("audio/mpeg"), //true
//             url: String::from("https://something.com/epi2"),
//             downloaded: 0, //false
//             podcast_id: 1,
//         };
//         let res = epi.delete_episode(_conn.conn).unwrap();
//         assert_eq!(res, 1);
//     }
// }
