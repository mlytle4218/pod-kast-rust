use rusqlite::{params, Connection, Error, Result};
use log::{error,info};

use super::category::Category;
use super::super::config::config::Config;
use super::data::DB;
use std::io::{self, Write};

use chrono::Utc;
use rss::Channel;
// use std::error::Error;

use crate::data::episode::Episode;

use super::super::utilities::utilities::{enter_info_util, error_message, enter_search_terms};
use rustyline::error::ReadlineError;

use super::super::menu::screen::Screen;
use super::super::api::api::AppleSearch;
use super::super::data;


#[derive(Debug)]
pub struct Podcast {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub audio: String,
    pub video: String,
    pub category_id: i32,
    pub collection_id: i32
}
impl Podcast {
    pub fn new() -> Podcast {
        info!("New Podcast!");
        let config: Config = Config::new();
        Podcast {
            id: 0,
            name: String::from(""),
            url: String::from(""),
            audio: config.pi.def_audio_loc.clone(),
            video: config.pi.def_video_loc.clone(),
            category_id: -1,
            collection_id: 1
        }
    }
    fn choose_category_for_podcast(&self) -> Result<i32, Error> {
        match Category::new().read_all_categories() {
            Ok(cats) =>{
                let cats_len: i32 = cats.len() as i32; 
                println!("\x1B[2J\x1B[1;1H");
                println!("Set Category for {}", self.name);
                for (i, ct) in cats.iter().enumerate() {
                    println!("number {}. {}",(i+1),ct.name);
                }
                
                info!("update new podcast category id");
                let mut line = String::new();
                print!("{}: ", "Choose from existing Categories");
                io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut line).unwrap();
                line.pop();
                match line.trim().parse::<i32>() {
                    Ok(val) => {
                        if val <= cats_len  && val > 0 {
                            Ok(val)
                        } else {
                            return Err(Error::InvalidQuery)
                        }
                    }
                    Err(_) => return Err(Error::InvalidQuery)

                }
            },
            Err(e) => {
                return Err(e)
            }
        }

    }
    fn choose_download_for_podcast(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
        match enter_info_util("Audio Download Location: ", self.audio.as_str()) {
            Ok(mut audio_dnld_loc) =>{
                match audio_dnld_loc.chars().last() {
                    Some(result) =>{
                        match result {
                            '/' =>{
                                // nada
                            },
                            _ => {
                                audio_dnld_loc.push('/');
                            }
                        }
                    },
                    None => {
                        error!("last char was not found");
                    }
                }
                self.audio = audio_dnld_loc;
                match enter_info_util("Video Download Location: ", self.video.as_str())  {
                    Ok(mut video_dnlod_loc) => {
                        match video_dnlod_loc.chars().last() {
                            Some(result) =>{
                                match result {
                                    '/' =>{
                                        // nada
                                    },
                                    _ => {
                                        video_dnlod_loc.push('/');
                                    }
                                }
                            },
                            None => {
                                error!("last char was not found");
                            }
                           
                        }
                        self.video = video_dnlod_loc;
                        return Ok(1)
                    },
                    Err(e) => return Err(Box::new(e))
                }

            },
            Err(e) => return Err(Box::new(e))
        }
    }
    pub fn save_existing(&mut self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        if self.category_id == -1 {
            match self.choose_category_for_podcast() {
                Ok(category) =>{
                    self.category_id = category
                },
                Err(e) => return Err(e)
            }
            match self.choose_download_for_podcast() {
                Ok(_) =>{
                    info!("podcast download options updated");
                },
                Err(e) => {
                    error!("{}",e);
                    return Err(data::podcast::Error::InvalidQuery)
                }
            }
        }
        let result: usize = conn.execute(
            "INSERT INTO podcasts (name, url, audio, video, category_id, collection_id, viewed) VALUES (?1, ?2, ?3,?4, ?5, ?6, ?7)",
            params![self.name, self.url, self.audio, self.video, self.category_id, self.collection_id, 0],
        )?;
        self.id  = conn.last_insert_rowid();
        



        match  self.retreive_episodes() {
            Ok(episodes) =>{
                println!("downloading episodes for {}.", self.name);
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




        Ok(result)
    }
    pub fn delete_existing(&mut self) -> Result<(), Error> {
        let db: DB = DB::new(Config::new());
        let mut conn: Connection = db.connect_to_database();
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM episodes where podcast_id=(?1);", params![self.id])?;
        tx.execute("DELETE FROM podcasts where podcast_id=(?1);", params![self.id])?;
        let result  = tx.commit().unwrap();
        Ok(result)
    }
    pub fn update_existing(&mut self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let result = conn.execute(
            "UPDATE podcasts SET name=(?1), url=(?2), audio=(?3), video=(?4), category_id=(?5) where podcast_id=(?6)",
            params![self.name, self.url, self.audio, self.video, self.category_id, self.id],
        )?;
        Ok(result)
    }
    pub fn read_all_podcasts2(cat: Option<String>) ->Result<Vec<Podcast>, Error>  {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let mut results: Vec<Podcast> = Vec::new();
        let full_statement: String;
        match cat {
            Some(cat_ref) =>{
                full_statement = format!("SELECT * FROM podcasts  where category_id={};", cat_ref);
            },
            None => {
                full_statement = format!("SELECT * FROM podcasts;");
            }
        }
        
        let mut _stmt = match conn.prepare(&full_statement){
            Ok(mut prepared_statement) => {
                match prepared_statement.query_map([], |row| {
                    Ok(Podcast {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        url: row.get(2)?,
                        audio: row.get(3)?,
                        video: row.get(4)?,
                        category_id: row.get(5)?,
                        collection_id: row.get(6)?
                    })
                }) {
                    Ok(pod_itr) =>{
                        for category in pod_itr {
                            results.push(category.unwrap());
                        }
                        return Ok(results)

                    },
                    Err(e) => return Err(e)
                }

            },
            Err(e) => return Err(e)
        };
    }
    pub fn create_podcast_pod() {
        println!("\x1B[2J\x1B[1;1H");
        match Podcast::edit_podcast_details_pod(Podcast::new()) {
            Ok(mut podcast) =>{
                match podcast.save_existing() {
                    Ok(_) =>{
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
    fn edit_podcast_details_pod(mut pod: Podcast) -> Result<Podcast, ReadlineError> {
        match enter_info_util("Podcast name: ", &pod.name) {
            Ok(name) => {
                pod.name = name;
                info!("New Podcast name: {}", pod.name);
                match enter_info_util("Podcast URL: ", &pod.url) {
                    Ok(url) => {
                        pod.url = url;
                        info!("New Podcast URL: {}", pod.url);
                        match enter_info_util("Default Audio Save location : ", &pod.audio) {
                            Ok(audio) =>{
                                pod.audio = audio;
                                info!("New Podcast default audio location: {}", pod.audio);
                                match enter_info_util("Default Audio Save location : ", &pod.video) {
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
    pub fn edit_podcast_pod() {
        println!("\x1B[2J\x1B[1;1H");
        match Podcast::read_all_podcasts2(None) {
            Ok(res) =>{
                match Self::display_pods_single_result_pod(&res) {   
                    Ok(chosen) =>{
                        match Podcast::edit_podcast_details_pod(res[(chosen as usize)-1].clone()) {
                            Ok(mut podcast) =>{
                                match podcast.update_existing() {
                                    Ok(_) => {
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
    fn display_pods_single_result_pod(pods: &Vec<Podcast>) -> Result<u16, Error> {
        let screen = Screen::new();
        let pods_len = pods.len(); 
        // let mut results: u16;
        if pods.len() == 0 {
            error_message(format!("No Podcasts to display.").as_str());
            return Err(Error::InvalidColumnName("".to_string()))
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
                println!("number {}. {}", (row_iter + 1), pods[row_iter as usize].name);
                row_iter += 1;
            }
            let mut line = String::new();
            print!("Choice: ");
            io::stdout().flush().unwrap();
            match std::io::stdin().read_line(&mut line) {
                Ok(_) =>{
                    match line.trim_end_matches('\n') {
                        "q" => return Err(Error::InvalidColumnName("".to_string())),
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
                },
                Err(e) =>{
                    error!("{}",e);
                }
            }
    
        }
    }
    pub fn delete_podcast_pod() {
        match Podcast::read_all_podcasts2(None) {
            Ok(pods) => {
                match Podcast::display_pods2(&pods) {
                    Ok(chosen) => {
                        for mut podcast in chosen {
                            match podcast.delete_existing() {
                                Ok(_) => {
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
            // info!("Display pods");
            let start = page_iter*display_size;
            let end; // = 0;
    
            if ((page_iter+1)*display_size)-1 < (pods_len as u16) - 1 {
                end = ((page_iter+1)*display_size)-1;
            } else {
                end = (pods_len as u16) - 1;
            }
    
            row_iter = start;
            while row_iter <= end {
                println!("number {}. {}", (row_iter + 1), pods[row_iter as usize].name);
                row_iter += 1;
            }
            let mut line = String::new();
            print!("Choice: ");
            io::stdout().flush().unwrap();
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => {
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
                },
                Err(e) => {
                    error!("{}",e);
                }
    
            }
    
        }
    }
    pub fn search() {
        let terms = enter_search_terms();
        let search = AppleSearch::new("https://itunes.apple.com".to_string(),terms.to_string(),100);
        match search.search() {
            Ok(res) =>{
                match Podcast::display_pods2(&res) {
                    Ok(chosen) => {
                        for mut each in chosen {
                            match each.save_existing() {
                                Ok(_) =>{
                                    info!("{} saved", each.name);
                                },
                                Err(e) =>{
                                    error!("{}",e);
                                }
                            }
                        }
                    },
                    Err(e) =>{
                        error!("{}",e);
                    }
                }
            },
            Err(e) => {
                error!("{}", e);
            }
        }
    }
    fn display_pods_to_choose_episodes_archive2(pods: &Vec<Podcast>) -> Result<Podcast, Error> {
        let screen = Screen::new();
        let pods_len = pods.len(); 
        // let result: i16 = -1;
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
                        println!("number {}. {} - {}", (row_iter + 1), pods[row_iter as usize].name, count);
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
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => {
                    match line.trim_end_matches('\n') {
                        "q" => return Err(Error::QueryReturnedNoRows),
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
                            // info!("display_pods not q, n, or p");
                            match line.trim().parse::<i16>() {
                                Ok(line_parsed) => {
                                    return Ok(pods[(line_parsed as usize)-1].clone())
                                },
                                Err(_) => {
                                    error_message(format!("{} is not a valid number.", line.trim()).as_str());
                                    return Err(Error::QueryReturnedNoRows)
                                }
            
                            }
                        }
                    }
                },
                Err(e) => {
                    error!("{}",e);
                }
            }
    
        }
    }
    pub fn archive() {
        match Podcast::read_all_podcasts2(None) {
            Ok(pods) =>{
                loop {
                    match Podcast::display_pods_to_choose_episodes_archive2(&pods) {
                        Ok(chosen) => {
                            match Episode::new().read_all_episodes_by_podcast_id(chosen.id, Some(1 as i64)) {
                                Ok(all_episodes) =>{
                                    match Episode::display_episodes_epi(&all_episodes) {
                                        Ok(chosen_episodes) =>{
                                            for episode in chosen_episodes {
                                                match episode.add_to_download_queue() {
                                                    Ok(_) =>{
                                                        info!("{} added to download queue", episode.title)
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

    #[tokio::main]
    pub async fn retreive_episodes(&self) -> Result<Vec<Episode>, Box<dyn std::error::Error>> {
        let content = reqwest::get(self.url.clone()).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;

        let mut episode_vec: Vec<Episode> = Vec::new();
        for it in channel.items() {
            match it.enclosure.as_ref() {
                Some(en) =>{
                    match String::from(&en.length).parse::<i32>() {
                        Ok(temp_length) =>{
                            episode_vec.push(Episode {
                                id: 0,
                                title: String::from(it.title.as_ref().unwrap()),
                                published: Utc::now(),
                                summary: String::from(it.description.as_ref().unwrap()),
                                length: temp_length,
                                audio: String::from(&en.mime_type),
                                url: String::from(&en.url),
                                downloaded: 0,
                                viewed: 0,
                                podcast_id: self.id as i16,
                                queue: 0
                            });
                        },
                        Err(e) => {
                            error!("{}", e);
                            continue
                        }
                    }
                },
                None =>{
                    error!("Rss enclosure empty.");
                    continue
                }
            }
        }
        Ok(episode_vec)
    }
}

impl Clone for Podcast {
    fn clone(&self) -> Podcast {
        Podcast {
            id: self.id.clone(),
            name: self.name.clone(),
            url: self.url.clone(),
            audio: self.audio.clone(),
            video: self.video.clone(),
            category_id: self.category_id.clone(),
            collection_id: self.collection_id.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct LocalTestContext {
        pub conn: Connection,
    }
    impl LocalTestContext {
        fn new() -> LocalTestContext {
            let _conn = TestContext::new().conn;
            _conn
                .execute(
                    "INSERT INTO categories (name) VALUES (?1)",
                    params![String::from("News")],
                )
                .unwrap();
            let pod = Podcast {
                id: 0,
                name: String::from("Episode 1"),
                url: String::from("https://somthing.com"),
                audio: String::from("/home/marc/audio"),
                video: String::from("/home/marc/video"),
                category_id: 1,
            };
            _conn
                .execute(
                    "INSERT INTO podcasts (name, url, audio, video, category_id) VALUES (?1, ?2, ?3,?4, ?5)",
                    params![pod.name, pod.url, pod.audio, pod.video, pod.category_id],
                )
                .unwrap();
            let epi = Episode {
                id: 0,
                title: String::from("Episode"),
                published: Utc::now(),
                summary: String::from("Stuff about Episode 1"),
                length: 3600,
                audio: String::from("audio/mpeg"), //true
                url: String::from("https://something.com/epi1"),
                downloaded: 0, //false
                podcast_id: 1,
            };
            _conn.execute(
                "INSERT INTO episodes (title, published, summary, length, audio, url, downloaded, podcast_id) VALUES (?1, ?2, ?3,?4,?5,?6,?7,?8)",
                params![
                    epi.title,
                    epi.published,
                    epi.summary,
                    epi.length,
                    epi.audio,
                    epi.url,
                    epi.downloaded,
                    epi.podcast_id
                    ],
            ).unwrap();
            LocalTestContext { conn: _conn }
        }
    }
    use super::super::context::TestContext;
    use super::*;

    #[test]
    fn test_create_podcast() {
        let _conn = LocalTestContext::new();
        let mut pod = Podcast {
            id: 0,
            name: String::from("Podcast2 2"),
            url: String::from("https://somthing2.com"),
            audio: String::from("/home/marc/audio2"),
            video: String::from("/home/marc/video2"),
            category_id: 1,
        };
        let res = pod.create_podcast(_conn.conn).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_read_podcast() {
        let _conn = LocalTestContext::new();
        let pod = Podcast::new();
        let _res = pod.read_podcasts(_conn.conn).unwrap();
        assert_eq!(_res.len(), 1);
        assert_eq!(_res[0].id, 1);
    }

    #[test]
    fn test_update_podcast() {
        let _conn = LocalTestContext::new();
        let pod = Podcast {
            id: 1,
            name: String::from("Podcast 2 update"),
            url: String::from("https://something2.com"),
            audio: String::from("/home/marc/audio2"),
            video: String::from("/home/marc/video2"),
            category_id: 1,
        };
        let res = pod.update_podcast(_conn.conn).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_delete_podcast() {
        let mut _conn = LocalTestContext::new();
        let pod = Podcast {
            id: 1,
            name: String::from("Podcast 2 update"),
            url: String::from("https://something2.com"),
            audio: String::from("/home/marc/audio2"),
            video: String::from("/home/marc/video2"),
            category_id: 1,
        };
        let res = pod.delete_podcast(&mut _conn.conn).unwrap();
        assert_eq!(res, ());
    }
}
