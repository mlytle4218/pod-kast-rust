use rusqlite::{ Connection, Error, Result };
// use std::io::prelude::*;
// use std::fs::File;

// fn trial() -> Result<(), Error> {
//     let conn = Connection::open("assets/pod-kast.db")?;
//     conn.execute("SELECT * FROM podcasts;", NO_PARAMS).unwrap();
//     Ok(())
// }

fn get_categories(conn: Connection) -> Result<(), Error> {
    let c = conn.execute("SELECT * FROM categories;", []).unwrap();
    println!("{:?}", c);
    Ok(())
}

#[derive(Debug)]
struct TestContext {
    conn: Connection,
}

impl TestContext {

    fn create() -> Result<Connection> {
        let _conn = Connection::open_in_memory()?;
        _conn.execute("PRAGMA foreign_keys=OFF;
        BEGIN TRANSACTION;
        CREATE TABLE podcasts (
            podcast_id INTEGER NOT NULL, 
            name VARCHAR(250), 
            url VARCHAR(250), 
            audio VARCHAR(250), 
            video VARCHAR(250), category varchar(250), 
            PRIMARY KEY (podcast_id)
        );
        CREATE TABLE episodes (
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
        );
        CREATE TABLE categories (
            category_id INTEGER NOT NULL, 
            category VARCHAR(250), 
            PRIMARY KEY (category_id)
        );
        COMMIT;", [])?;
        Ok(_conn)
    }
    fn new() -> TestContext {
        let _conn = TestContext::create().unwrap();
        TestContext {
            conn: _conn
        }
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        println!("clean up resources");
    }
}

#[test]
fn try_it() {
    let conn = Connection::open(":memory:").unwrap();
    let _ctx = TestContext::new();
    let _res = get_categories(_ctx.conn);
    assert_eq!(_res, String::from("hiya"));
    
    // get_categories(conn);
}