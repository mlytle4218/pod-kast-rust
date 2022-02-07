use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, Error, Result};

use super::podcast::Podcast;

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
    pub podcast_id: i16,
}
impl Episode {
    fn new() -> Episode {
        Episode {
            id: 0,
            title: String::from("nada"),
            published: Utc::now(),
            summary: String::from("nada"),
            length: 3600,
            audio: String::from("audio/mpeg"),
            url: String::from("nada"),
            downloaded: 0,
            podcast_id: 1,
        }
    }

    fn create_episode(&mut self, conn: Connection) -> Result<usize, Error> {
        let result = conn.execute(
            "INSERT INTO episodes (title, published, summary, length, audio, url, downloaded, podcast_id) VALUES (?1, ?2, ?3,?4, ?5, ?6, ?7, ?8)", 
            params![self.title, self.published, self.summary,self.length, self.audio, self.url, self.downloaded, self.podcast_id]
        )?;
        self.id = conn.last_insert_rowid();
        Ok(result)
    }

    fn read_episodes(&self, conn: Connection) -> Result<Vec<Episode>, Error> {
        let mut stmt = conn.prepare("SELECT * FROM episodes;")?;
        let cat_iter = stmt.query_map([], |row| {
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
            })
        })?;
        let mut results: Vec<Episode> = Vec::new();
        for category in cat_iter {
            results.push(category.unwrap());
        }
        Ok(results)
    }

    fn update_episode(&self, conn: Connection) -> Result<usize, Error> {
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
        let result = conn.execute(
            "UPDATE episodes SET title=(?1), published=(?2), summary=(?3), length=(?4), audio=(?5), url=(?6), downloaded=(?7), podcast_id=(?8) where episode_id=(?9)", 
            params![self.title, self.published, self.summary, self.length, self.audio, self.url, self.downloaded, self.podcast_id, self.id]
        )?;
        Ok(result)
    }

    fn delete_episode(&self, conn: Connection) -> Result<usize, Error> {
        let result = conn.execute("DELETE FROM episodes where episode_id=(?1)", params![self.id])?;
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
