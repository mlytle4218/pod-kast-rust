use rusqlite::{params, Connection, Error, Result};



#[derive(Debug)]
pub struct Podcast {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub audio: String,
    pub video: String
}

fn create_podcast(conn: Connection, pod: Podcast) -> Result<usize, Error> {
    let result = conn.execute(
        "INSERT INTO podcasts (name, url, audio, video) VALUES (?1, ?2, ?3,?4)", 
        params![pod.name, pod.url, pod.audio, pod.video]
    )?;
    Ok(result)
}

fn read_podcasts(conn: Connection) -> Result<Vec<Podcast>, Error> {
    let mut stmt = conn.prepare("SELECT * FROM podcasts;")?;
    let cat_iter = stmt.query_map([], |row| {
        Ok(Podcast {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            audio: row.get(3)?,
            video: row.get(4)?
        })
    })?;
    let mut results: Vec<Podcast> = Vec::new();
    for category in cat_iter {
        results.push(category.unwrap());
    }
    Ok(results)
}

fn update_podcast_by_id(conn: Connection, pod: Podcast) -> Result<usize, Error> {
    let result = conn.execute("UPDATE podcasts SET name=(?1), url=(?2), audio=(?3), video=(?4)  where podcast_id=(?5)", params![pod.name, pod.url, pod.audio, pod.video, pod.id])?;
    Ok(result)
}

fn delete_podcast_by_id(conn: Connection, pod: Podcast) -> Result<usize, Error> {
    let result = conn.execute("DELETE FROM podcasts where podcast_id=(?1)", params![pod.id])?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::context::TestContext;

    #[test]
    fn test_create_podcast() {
        let _conn = TestContext::new();
        let pod = Podcast {
            id: 0,
            name: String::from("Podcast2 2"),
            url: String::from("https://somthing2.com"),
            audio: String::from("/home/marc/audio2"),
            video: String::from("/home/marc/video2")
        };
        let res = create_podcast(
            _conn.conn, 
            pod
        ).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_read_podcast() {
        let _conn = TestContext::new();
        let _res = read_podcasts(_conn.conn).unwrap();
        assert_eq!(_res.len(), 1);
        assert_eq!(_res[0].id, 1);
    }

    #[test]
    fn test_update_podcast_by_id() {
        let _conn = TestContext::new();
        let pod = Podcast {
            id: 1,
            name: String::from("Podcast 2 update"),
            url: String::from("https://something2.com"),
            audio: String::from("/home/marc/audio2"),
            video: String::from("/home/marc/video2")
        };
        let res = update_podcast_by_id(_conn.conn, pod).unwrap();
        assert_eq!(res, 1);
    }

    // #[test]
    // fn test_delete_podcast_by_id() {
    //     let _conn = TestContext::new();        
    //     let pod = Podcast {
    //         id: 1,
    //         name: String::from("Podcast 2 update"),
    //         url: String::from("https://something2.com"),
    //         audio: String::from("/home/marc/audio2"),
    //         video: String::from("/home/marc/video2")
    //     };
    //     let res = delete_podcast_by_id(_conn.conn, pod).unwrap();
    //     assert_eq!(res, 1);
    // }
}
