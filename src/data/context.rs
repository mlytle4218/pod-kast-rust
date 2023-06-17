// use rusqlite::{Connection, Result};

// #[derive(Debug)]
// pub struct TestContext {
//     pub conn: Connection,
// }

// impl TestContext {
//     fn create() -> Result<Connection> {
//         let _conn = Connection::open_in_memory()?;
//         _conn.execute(
//             "CREATE TABLE categories (
//                 category_id INTEGER PRIMARY KEY,
//                 name TEXT NOT NULL
//             )",
//             [],
//         )?;
//         _conn.execute(
//             "CREATE TABLE podcasts (
//                 podcast_id INTEGER NOT NULL, 
//                 name VARCHAR(250), 
//                 url VARCHAR(250), 
//                 audio VARCHAR(250), 
//                 video VARCHAR(250),
//                 category_id INTEGER,
//                 PRIMARY KEY (podcast_id), 
//                 FOREIGN KEY(category_id) REFERENCES categories (category_id)
//             )",
//             [],
//         )?;
//         _conn.execute(
//             "CREATE TABLE episodes (
//                 episode_id INTEGER NOT NULL, 
//                 title VARCHAR(100), 
//                 published DATETIME, 
//                 summary VARCHAR(500), 
//                 length INTEGER, 
//                 audio INTEGER, 
//                 url VARCHAR(250), 
//                 downloaded INTEGER, 
//                 podcast_id INTEGER, 
//                 viewed Integer, 
//                 PRIMARY KEY (episode_id), 
//                 FOREIGN KEY(podcast_id) REFERENCES podcasts (podcast_id)
//             )",
//             [],
//         )?;
//         Ok(_conn)
//     }
//     pub fn new() -> TestContext {
//         let _conn = TestContext::create().unwrap();
//         TestContext { conn: _conn }
//     }
// }
