use rusqlite::{params, Connection, Error, Result};
use chrono::{DateTime, Duration, Utc};

use super::podcast::Podcast;

#[derive(Debug)]
pub struct Episode {
    pub id: i32,
    pub title: String,
    pub published: DateTime<Utc>,
    pub summary: String,
    pub length: i32,
    pub audio: i8,
    pub url: String,
    pub downloaded: i8,
    pub podcast_id: i16
}

 
// fn create_episode(conn: Connection, pod: Episode) -> Result<usize, Error> {
//     let result = conn.execute(
//         "INSERT INTO podcasts (name, url, audio, video) VALUES (?1, ?2, ?3,?4)", 
//         params![pod.name, pod.url, pod.audio, pod.video]
//     )?;
//     Ok(result)
// }

fn read_episodes(conn: Connection) -> Result<Vec<Episode>, Error> {
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
            podcast_id: row.get(8)?
        })
    })?;
    let mut results: Vec<Episode> = Vec::new();
    for category in cat_iter {
        results.push(category.unwrap());
    }
    Ok(results)
}

// fn update_episode_by_id(conn: Connection, pod: Episode) -> Result<usize, Error> {
//     let result = conn.execute("UPDATE podcasts SET name=(?1), url=(?2), audio=(?3), video=(?4)  where podcast_id=(?5)", params![pod.name, pod.url, pod.audio, pod.video, pod.id])?;
//     Ok(result)
// }

// fn delete_episode_by_id(conn: Connection, pod: Episode) -> Result<usize, Error> {
//     let result = conn.execute("DELETE FROM podcasts where podcast_id=(?1)", params![pod.id])?;
//     Ok(result)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::context::TestContext;


    // #[test]
    // fn test_create_episode() {
    //     let _conn = TestContext::new();
    //     let pod = Episode {
    //         id: 0,
    //         name: String::from("Podcast2 2"),
    //         url: String::from("https://somthing2.com"),
    //         audio: String::from("/home/marc/audio2"),
    //         video: String::from("/home/marc/video2")
    //     };
    //     let res = create_episode(
    //         _conn.conn, 
    //         pod
    //     ).unwrap();
    //     assert_eq!(res, 1);
    // }

    #[test]
    fn test_read_episode() {
        let _conn = TestContext::new();
        let _res = read_episodes(_conn.conn).unwrap();
        assert_eq!(_res.len(), 1);
        assert_eq!(_res[0].id, 1);
    }

    // #[test]
    // fn test_update_episode_by_id() {
    //     let _conn = TestContext::new();
    //     let pod = Episode {
    //         id: 1,
    //         name: String::from("Episode 2 update"),
    //         url: String::from("https://something2.com"),
    //         audio: String::from("/home/marc/audio2"),
    //         video: String::from("/home/marc/video2")
    //     };
    //     let res = update_episode_by_id(_conn.conn, pod).unwrap();
    //     assert_eq!(res, 1);
    // }

    // #[test]
    // fn test_delete_episode_by_id() {
    //     let _conn = TestContext::new();        
    //     let pod = Episode {
    //         id: 1,
    //         name: String::from("Episode 2 update"),
    //         url: String::from("https://something2.com"),
    //         audio: String::from("/home/marc/audio2"),
    //         video: String::from("/home/marc/video2")
    //     };
    //     let res = delete_episode_by_id(_conn.conn, pod).unwrap();
    //     assert_eq!(res, 1);
    // }
}
