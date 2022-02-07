use chrono::{Utc};
use rusqlite::{params, Connection, Error, Result};

use super::episode::Episode;

#[derive(Debug)]
pub struct Podcast {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub audio: String,
    pub video: String,
    pub category_id: i32,
}
impl Podcast {
    pub fn new() -> Podcast {
        Podcast {
            id: 0,
            name: String::from("nada"),
            url: String::from("nada"),
            audio: String::from("nada"),
            video: String::from("nada"),
            category_id: 1,
        }
    }

    fn create_podcast(&mut self, conn: Connection) -> Result<usize, Error> {
        let mut result = conn.execute(
            "INSERT INTO podcasts (name, url, audio, video) VALUES (?1, ?2, ?3,?4)",
            params![self.name, self.url, self.audio, self.video],
        )?;
        self.id  = conn.last_insert_rowid();
        Ok(result)
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
