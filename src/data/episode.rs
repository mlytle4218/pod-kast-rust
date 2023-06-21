use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Error, Result};

use super::super::config::config::Config;
use super::data::DB;

use std::clone::Clone;

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
    pub fn count_episodes(&self, cat_id: i64) -> Result<usize, Error> {
        let stmt: String = format!("SELECT COUNT(viewed) from episodes where podcast_id={} and viewed=0;", cat_id);
        self.count_helper(&(stmt.clone()))
    }
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
    pub fn get_podcast_download_location(&self) -> Result<String, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let mut statement: String =format!("Select audio from podcasts where podcast_id={};", self.podcast_id);
        if self.audio.contains("video") {
            statement = format!("Select audio from podcasts where podcast_id={};", self.podcast_id);
        } 

        let _x = match conn.prepare(&statement.clone()) {
            Ok(mut prepared_statement) =>{
                match prepared_statement.query_map([],|row| {
                    Ok(Download {
                        result: row.get(0)?
                    })
                }) {
                    Ok( mut epi_itr) => {
                        match epi_itr.next() {
                            Some(download) =>{
                                match download {
                                    Ok(result) =>{
                                        return Ok(result.result)
                                    },
                                    Err(e) =>{
                                        return Err(e)
                                    }
                                }
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
                title: String::from("Episode 1"),
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
    fn test_create_episode() {
        let _conn = LocalTestContext::new();
        let mut epi = Episode {
            id: 0,
            title: String::from("Episode 2"),
            published: Utc::now(),
            summary: String::from("Stuff about Episode 2"),
            length: 3600,
            audio: String::from("audio/mpeg"), //true
            url: String::from("https://something.com/epi2"),
            downloaded: 0, //false
            podcast_id: 1,
        };
        let res = epi.create_episode(_conn.conn).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_read_episode() {
        let _conn = LocalTestContext::new();
        let epi = Episode::new();
        let _res = epi.read_episodes(_conn.conn).unwrap();
        assert_eq!(_res.len(), 1);
        assert_eq!(_res[0].id, 1);
    }

    #[test]
    fn test_update_episode() {
        let _conn = LocalTestContext::new();
        let epi = Episode {
            id: 1,
            title: String::from("Episode 2-new"),
            published: Utc::now(),
            summary: String::from("Stuff about Episode 2"),
            length: 3600,
            audio: String::from("audio/mpeg"), //true
            url: String::from("https://something.com/epi2"),
            downloaded: 0, //false
            podcast_id: 1,
        };
        let res = epi.update_episode(_conn.conn).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_delete_episode() {
        let _conn = LocalTestContext::new();
        let epi = Episode {
            id: 1,
            title: String::from("Episode 2-new"),
            published: Utc::now(),
            summary: String::from("Stuff about Episode 2"),
            length: 3600,
            audio: String::from("audio/mpeg"), //true
            url: String::from("https://something.com/epi2"),
            downloaded: 0, //false
            podcast_id: 1,
        };
        let res = epi.delete_episode(_conn.conn).unwrap();
        assert_eq!(res, 1);
    }
}
