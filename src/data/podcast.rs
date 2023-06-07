use chrono::{Utc};
use rusqlite::{params, Connection, Error, Result};
use log::{debug, error, log_enabled, info, Level};

use super::episode::Episode;
use super::category::Category;
use super::super::config::config::Config;
use super::data::DB;
use std::io::{self, Read, Write, BufRead};

#[derive(Debug, Clone)]
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
            audio: config.def_audio_loc.clone(),
            video: config.def_video_loc.clone(),
            category_id: -1,
            collection_id: 1
        }
    }

    pub fn save_existing(&mut self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        info!("self.category_id: {}", self.category_id);
        if self.category_id == -1 {
            info!("self.category_id == -1");
            // let cats: Vec<Category> = Category::read_all_categories();
            let temp_cat: Category = Category::new();
            match temp_cat.read_all_categories() {
                Ok(cats) =>{
                    let cats_len: usize = cats.len(); 
                    info!("{:?}", cats);
                    println!("\x1B[2J\x1B[1;1H");
                    println!("Set Category for {}", self.name);
                    for (i, ct) in cats.iter().enumerate() {
                        println!("{}. {}",(i+1),ct.name);
                    }
                    
                    info!("update new podcast category id");
                    let mut line = String::new();
                    print!("{}: ", "Choose from existing Categories");
                    io::stdout().flush().unwrap();
                    std::io::stdin().read_line(&mut line).unwrap();
                    line.pop();
                    match line.trim().parse::<usize>() {
                        // "q" => continue,


                        Ok(val) => {
                            if val <= cats_len  && val > 0 {
                                self.category_id = val as i32;
                                info!("{:?}", self);
                                // info!("{:?}", self.update_existing());
                            }
                        }
                        Err(_) => {
                            match line.trim() {
                                "q" => {},
                                _err => {}
                            }
                        }

                    }
                    info!("chosen category: {}", line);
                },
                Err(e) => {
                    error!("{}", e)
                }
            }


        }
        let mut result: usize = conn.execute(
            "INSERT INTO podcasts (name, url, audio, video, category_id, collection_id, viewed) VALUES (?1, ?2, ?3,?4, ?5, ?6, ?7)",
            params![self.name, self.url, self.audio, self.video, self.category_id, self.collection_id, 0],
        )?;
        self.id  = conn.last_insert_rowid();
        conn.close();
        Ok(result)
        // Ok(1 as usize)
    }

    pub fn delete_existing(&mut self) -> Result<(), Error> {
        let db: DB = DB::new(Config::new());
        let mut conn: Connection = db.connect_to_database();
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM episodes where podcast_id=(?1);", params![self.id])?;
        tx.execute("DELETE FROM podcasts where podcast_id=(?1);", params![self.id])?;
        let result  = tx.commit().unwrap();
        conn.close();
        Ok(result)
    }

    fn update_existing(&mut self) -> Result<(usize), Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let result = conn.execute(
            "UPDATE podcasts SET name=(?1), url=(?2), audio=(?3), video=(?4), category_id=(?5) where podcast_id=(?6)",
            params![self.name, self.url, self.audio, self.video, self.category_id, self.id],
        )?;
        conn.close();
        Ok(result)
    }

    pub fn create_podcast(&mut self, conn: Connection) -> Result<usize, Error> {
        let mut result = conn.execute(
            "INSERT INTO podcasts (name, url, audio, video) VALUES (?1, ?2, ?3,?4)",
            params![self.name, self.url, self.audio, self.video],
        )?;
        Ok(result)
    }
    pub fn read_all_podcasts(&self) ->Result<Vec<Podcast>, Error>  {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let mut stmt = conn.prepare("SELECT * FROM podcasts;")?;
        let pod_iter = stmt.query_map([], |row| {
            Ok(Podcast {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                audio: row.get(3)?,
                video: row.get(4)?,
                category_id: row.get(5)?,
                collection_id: row.get(6)?
            })
        })?;
        let mut results: Vec<Podcast> = Vec::new();
        for category in pod_iter {
            results.push(category.unwrap());
        }
        Ok(results)
    }
    pub fn read_all_podcasts_by_category(&self, cat: String) ->Result<Vec<Podcast>, Error>  {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let mut stmt = conn.prepare("SELECT * FROM podcasts  where category_id=(?);")?;
        let pod_iter = stmt.query_map([cat], |row| {
            Ok(Podcast {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                audio: row.get(3)?,
                video: row.get(4)?,
                category_id: row.get(5)?,
                collection_id: row.get(6)?
            })
        })?;
        let mut results: Vec<Podcast> = Vec::new();
        for category in pod_iter {
            results.push(category.unwrap());
        }
        Ok(results)
    }

    fn read_podcasts(&self, conn: Connection) -> Result<Vec<Podcast>, Error> {
        let mut stmt = conn.prepare("SELECT * FROM podcasts;")?;
        let cat_iter = stmt.query_map([], |row| {
            Ok(Podcast {
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                audio: row.get(3)?,
                video: row.get(4)?,
                category_id: row.get(5)?,
                collection_id: row.get(6)?
            })
        })?;
        let mut results: Vec<Podcast> = Vec::new();
        for category in cat_iter {
            results.push(category.unwrap());
        }
        Ok(results)
    }

    fn update_podcast(&self, conn: Connection) -> Result<usize, Error> {
        let result = conn.execute(
            "UPDATE podcasts SET name=(?1), url=(?2), audio=(?3), video=(?4), category_id=(?5) where podcast_id=(?6)",
            params![self.name, self.url, self.audio, self.video, self.category_id, self.id],
        )?;
        Ok(result)
    }

    fn delete_podcast(&self, conn: &mut Connection) -> Result<(), Error> {
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM episodes where podcast_id=(?1);", params![self.id])?;
        tx.execute("DELETE FROM podcasts where podcast_id=(?1);", params![self.id])?;
        let result  = tx.commit().unwrap();
        Ok(result)
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
