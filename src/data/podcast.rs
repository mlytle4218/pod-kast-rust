use rusqlite::{params, Connection, Error, Result};

#[derive(Debug)]
struct Podcast {
    id: i32,
    name: String,
    url: String,
    audio: String,
    video: String
}

fn create_podcast(conn: Connection, name: String, url: String, audio: String, video: String) -> Result<usize, Error> {
    let result = conn.execute(
        "INSERT INTO podcasts (name, url, audio, video) VALUES (?1, ?2, ?3,?4)", 
        params![name, url, audio, video]
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

fn update_podcast(conn: Connection, name: String, url: String, audio: String, video: String) -> Result<usize, Error> {
    let result = conn.execute(
        "UPDATE podcasts SET name=(?1), url=(?2), audio=(?3), video=(?4) where podcast_id=(?5)", 
        params![name, url, audio, video]
    )?;
    Ok(result)
}
// fn update_podcast_by_id(conn: Connection, id: i8, name: String) -> Result<usize, Error> {
//     let result = conn.execute("UPDATE podcasts SET name=(?1) where id=(?2)", params![name, id])?;
//     Ok(result)
// }

// fn delete_podcast_by_id(conn: Connection, id: i8) -> Result<usize, Error> {
//     let result = conn.execute("DELETE FROM podcasts where id=(?1)", params![id])?;
//     Ok(result)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestContext {
        conn: Connection,
    }

    impl TestContext {
        fn create() -> Result<Connection> {
            let _conn = Connection::open_in_memory()?;
            _conn.execute(
                "CREATE TABLE categories (
                      id              INTEGER PRIMARY KEY,
                      name            TEXT NOT NULL
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
                href VARCHAR(250), 
                downloaded INTEGER, 
                podcast_id INTEGER, veiwed Integer, 
                PRIMARY KEY (episode_id), 
                FOREIGN KEY(podcast_id) REFERENCES podcasts (podcast_id)
            )",
                [],
            )?;
            _conn.execute(
                "INSERT INTO podcasts (name, url, audio, video) VALUES (?1, ?2, ?3,?4)",
                params![
                    String::from("Podcast 1"),
                    String::from("https://somthing.com"),
                    String::from("/home/marc/audio"),
                    String::from("/home/marc/video"),
                    ],
            )?;
            Ok(_conn)
        }
        fn new() -> TestContext {
            let _conn = TestContext::create().unwrap();
            TestContext { conn: _conn }
        }
    }

    #[test]
    fn test_create_podcast() {
        let _conn = TestContext::new();
        let res = create_podcast(
            _conn.conn, 
            String::from("Podcast2 2"),
            String::from("https://somthing2.com"),
            String::from("/home/marc/audio2"),
            String::from("/home/marc/video2"),
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

    // #[test]
    // fn test_update_podcast_by_id() {
    //     let _conn = TestContext::new();
    //     let res = update_podcast_by_id(_conn.conn, 1, String::from("News-new")).unwrap();
    //     assert_eq!(res, 1);
    // }

    // #[test]
    // fn test_delete_podcast_by_id() {
    //     let _conn = TestContext::new();
    //     let res = delete_podcast_by_id(_conn.conn, 1).unwrap();
    //     assert_eq!(res, 1);
    // }
}
