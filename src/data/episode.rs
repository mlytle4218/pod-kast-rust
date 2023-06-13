use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Error, Result};
use log::error;

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
    
    pub fn count_episodes(&self, cat_id: i64) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        
        let mut stmt = conn.prepare("SELECT COUNT(viewed) from episodes where podcast_id=(?) and viewed=0;")?;
        // info!("stmt is :{:?}", cat_id);
        let epi_iter = stmt.query_map([cat_id], |row| {
            Ok(Count {
                result: row.get(0)?
            })
        })?;
        for each in epi_iter {
            match each {
                Ok(result) =>{
                    return Ok(result.result)
                },
                Err(e) => {
                    error!("{}",e)
                }
            }
        }
        //bad design will return zero if there is a problem not because it is zero. - fix this later
        Ok(0 as usize)
    }

    pub fn save_existing(&mut self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let result = conn.execute(
            "INSERT INTO episodes (title, published, summary, length, audio, url, downloaded, podcast_id,viewed) VALUES (?1, ?2, ?3,?4, ?5, ?6, ?7, ?8,?9)", 
            params![self.title, self.published, self.summary,self.length, self.audio, self.url, self.downloaded, self.podcast_id,0]
        )?;
        // let result = conn.execute(
        //     "BEGIN 
        //      IF NOT EXISTS (SELECT * FROM episodes 
        //         WHERE url = ?6)
        //         BEGIN 
        //             INSERT INTO episodes (title, published, summary, length, audio, url, downloaded, podcast_id) VALUES (?1, ?2, ?3,?4, ?5, ?6, ?7, ?8)
        //         END
        //     END", 
        //     params![self.title, self.published, self.summary,self.length, self.audio, self.url, self.downloaded, self.podcast_id]
        // )?;



        // BEGIN
        //     IF NOT EXISTS (SELECT * FROM EmailsRecebidos 
        //                     WHERE De = @_DE
        //                     AND Assunto = @_ASSUNTO
        //                     AND Data = @_DATA)
        //     BEGIN
        //         INSERT INTO EmailsRecebidos (De, Assunto, Data)
        //         VALUES (@_DE, @_ASSUNTO, @_DATA)
        //     END
        //     END



        // match conn.execute(
        //     "INSERT INTO episodes (title, published, summary, length, audio, url, downloaded, podcast_id) VALUES (?1, ?2, ?3,?4, ?5, ?6, ?7, ?8)", 
        //     params![self.title, self.published, self.summary,self.length, self.audio, self.url, self.downloaded, self.podcast_id]
        // ) {
        //     Ok(result) =>{
        //         self.id = conn.last_insert_rowid();
        //         Ok(result)
        //     },
        //     Err(e) => {
        //         error!("{}", e)
        //     }
        // }
        
        self.id = conn.last_insert_rowid();
        Ok(result)
    }
    // fn create_episode(&mut self, conn: Connection) -> Result<usize, Error> {
    //     let result = conn.execute(
    //         "INSERT INTO episodes (title, published, summary, length, audio, url, downloaded, podcast_id) VALUES (?1, ?2, ?3,?4, ?5, ?6, ?7, ?8)", 
    //         params![self.title, self.published, self.summary,self.length, self.audio, self.url, self.downloaded, self.podcast_id]
    //     )?;
    //     self.id = conn.last_insert_rowid();
    //     Ok(result)
    // }
    pub fn read_all_episodes_by_podcast_id(&self, pod_id: i64) ->Result<Vec<Episode>, Error>  {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let mut stmt = conn.prepare("SELECT * FROM episodes where podcast_id=(?) AND viewed=0 ORDER BY title ASC;")?;
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

    // fn read_episodes(&self, conn: Connection) -> Result<Vec<Episode>, Error> {
    //     let mut stmt = conn.prepare("SELECT * FROM episodes;")?;
    //     let cat_iter = stmt.query_map([], |row| {
    //         Ok(Episode {
    //             id: row.get(0)?,
    //             title: row.get(1)?,
    //             published: row.get(2)?,
    //             summary: row.get(3)?,
    //             length: row.get(4)?,
    //             audio: row.get(5)?,
    //             url: row.get(6)?,
    //             viewed: row.get(7)?,
    //             downloaded: row.get(8)?,
    //             podcast_id: row.get(9)?,
    //             queue: row.get(10)?
    //         })
    //     })?;
    //     let mut results: Vec<Episode> = Vec::new();
    //     for category in cat_iter {
    //         results.push(category.unwrap());
    //     }
    //     Ok(results)
    // }
    pub fn add_to_download_queue(&self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let result = conn.execute(
            "UPDATE episodes SET queue=1 where episode_id=(?1)", 
            params![self.id]
        )?;
        Ok(result)
    }
    pub fn mark_downloaded(&self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let result = conn.execute(
            "UPDATE episodes SET queue=0 AND downloaded=1 where episode_id=(?1)", 
            params![self.id]
        )?;
        Ok(result)
    }
    pub fn get_podcast_download_location(&self) -> Result<String, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        // let mut format: String = "".to_string();
        // cheeeeaaaaapppppp fix later
        let mut stmt; // = conn.prepare("Select (?) from podcasts where podcast_id=(?);")?;
        if self.audio.contains("audio") {
            stmt = conn.prepare("Select audio from podcasts where podcast_id=(?);")?;
        } else {
            stmt = conn.prepare("Select video from podcasts where podcast_id=(?);")?;
        }
        
        let epi_iter = stmt.query_map([self.podcast_id.to_string()], |row| {
        // let mut epi_iter = stmt.query_map([self.podcast_id.to_string()], |row| {
            Ok(Download {
                result: row.get(0)?
            }
        )
        })?;            

        for each in epi_iter {
            match each {
                Ok(result) =>{
                    return Ok(result.result.to_string())
                },
                Err(e) => {
                    error!("{}",e)
                }
            }
        }
        Ok("".to_string())

    }
    pub fn read_all_in_download_queue() -> Result<Vec<Episode>, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let mut stmt = conn.prepare("SELECT * FROM episodes where queue=1;")?;
        let epi_iter = stmt.query_map([], |row| {
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
        })?;
        let mut results: Vec<Episode> = Vec::new();
        for episode in epi_iter {
            results.push(episode.unwrap());
        }
        Ok(results)

    }
    pub fn remove_from_download_queue(&self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let result = conn.execute(
            "UPDATE episodes SET queue=0 where episode_id=(?1)", 
            params![self.id]
        )?;
        Ok(result)
    }
    pub fn mark_viewed(&self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let result = conn.execute(
            "UPDATE episodes SET viewed=1 where episode_id=(?1)", 
            params![self.id]
        )?;
        Ok(result)

    }

    // fn update_episode(&self, conn: Connection) -> Result<usize, Error> {
    //     // let epi = Episode {
    //     //     id: 0,
    //     //     title: String::from("Episode 1"),
    //     //     published: Utc::now(),
    //     //     summary: String::from("Stuff about Episode 1"),
    //     //     length: 3600,
    //     //     audio: String::from("audio/mpeg"), //true
    //     //     url: String::from("https://something.com/epi1"),
    //     //     viewed: 0, //false
    //     //     downloaded: 0, //false
    //     //     podcast_id: 1,
    //     //     queue: 0
    //     // };
    //     let result = conn.execute(
    //         "UPDATE episodes SET title=(?1), published=(?2), summary=(?3), length=(?4), audio=(?5), url=(?6), downloaded=(?7), podcast_id=(?8) where episode_id=(?9)", 
    //         params![self.title, self.published, self.summary, self.length, self.audio, self.url, self.downloaded, self.podcast_id, self.id]
    //     )?;
    //     Ok(result)
    // }

    // fn delete_episode(&self, conn: Connection) -> Result<usize, Error> {
    //     let result = conn.execute("DELETE FROM episodes where episode_id=(?1)", params![self.id])?;
    //     Ok(result)
    // }
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
