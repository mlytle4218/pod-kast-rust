use rusqlite::{params, Connection, Error, Result};
use chrono::{DateTime, Duration, Utc};

use super::podcast::Podcast;
use super::episode::Episode;

#[derive(Debug)]
pub struct TestContext {
    pub conn: Connection,
}

impl TestContext {
    fn create() -> Result<Connection> {
        let _conn = Connection::open_in_memory()?;
        _conn.execute(
            "CREATE TABLE categories (
                      id INTEGER PRIMARY KEY,
                      name TEXT NOT NULL
                      )",
            [],
        )?;
        _conn.execute(
            "CREATE TABLE podcasts (
                podcast_id INTEGER NOT NULL, 
                name VARCHAR(250), 
                url VARCHAR(250), 
                audio VARCHAR(250), 
                video VARCHAR(250), category varchar(250), 
                PRIMARY KEY (podcast_id)
            )",
            [],
        )?;
        _conn.execute(
            "CREATE TABLE episodes (
                episode_id INTEGER NOT NULL, 
                title VARCHAR(100), 
                published DATETIME, 
                summary VARCHAR(500), 
                length INTEGER, 
                audio INTEGER, 
                url VARCHAR(250), 
                downloaded INTEGER, 
                podcast_id INTEGER, veiwed Integer, 
                PRIMARY KEY (episode_id), 
                FOREIGN KEY(podcast_id) REFERENCES podcasts (podcast_id)
            )",
            [],
        )?;
        _conn.execute(
            "INSERT INTO categories (name) VALUES (?1)",
            params![String::from("News")],
        )?;
        let pod = Podcast {
            id: 0,
            name: String::from("Episode 1"),
            url: String::from("https://somthing.com"),
            audio: String::from("/home/marc/audio"),
            video: String::from("/home/marc/video"),
        };
        _conn.execute(
            "INSERT INTO podcasts (name, url, audio, video) VALUES (?1, ?2, ?3,?4)",
            params![pod.name, pod.url, pod.audio, pod.video],
        )?;
        let epi = Episode {
            id: 0,
            title: String::from("Episode"),
            published: Utc::now(),
            summary: String::from("Stuff about Episode 1"),
            length: 3600,
            audio: 1, //true
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
            )?;
        Ok(_conn)
    }
    pub fn new() -> TestContext {
        let _conn = TestContext::create().unwrap();
        TestContext { conn: _conn }
    }
}
