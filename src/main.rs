mod api {
    pub mod api;
}

mod menu {
    pub mod menu;
}

mod data {
    pub mod data;
    pub mod category;
    pub mod podcast;
}

use rusqlite::{ Connection, Error, Result, params };

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
        Ok(_conn)
    }
    fn new() -> TestContext {
        let _conn = TestContext::create().unwrap();
        TestContext {
            conn: _conn
        }
    }
}


fn read_categories(conn: Connection) -> Result<Vec<Category>, Error> {
    let mut stmt = conn.prepare("SELECT * FROM categories;")?;
    let cat_iter = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    let mut results: Vec<Category> = Vec::new();
    for category in cat_iter {
        results.push(category.unwrap());
        // println!("Found {:?}", category.unwrap());
    }
    // for result in results {
    //     println!("{:?}", result);
    // }
    Ok(results)
}

#[derive(Debug)]
struct Category {
    id: i32,
    name: String
}

fn main() -> Result<()> {

    let _conn = TestContext::new();
    // let _res = read_categories(_conn.conn).unwrap();

    let mut stmt = _conn.conn.prepare("SELECT * FROM categories;")?;
    let cat_iter = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    let mut results: Vec<Category> = Vec::new();
    for category in cat_iter {
        // results.push(category.unwrap());
        println!("Found {:?}", category.unwrap());
    }



    // let conn = Connection::open_in_memory()?;

    // conn.execute(
    //     "CREATE TABLE category (
    //               id              INTEGER PRIMARY KEY,
    //               name            TEXT NOT NULL
    //               )",
    //     [],
    // )?;

    // conn.execute(
    //     "CREATE TABLE podcasts (
    //         podcast_id INTEGER NOT NULL, 
    //         name VARCHAR(250), 
    //         url VARCHAR(250), 
    //         audio VARCHAR(250), 
    //         video VARCHAR(250), category varchar(250), 
    //         PRIMARY KEY (podcast_id)
    //     )",
    //     [],
    // )?;

    // conn.execute(
    //     "CREATE TABLE episodes (
    //         episode_id INTEGER NOT NULL, 
    //         title VARCHAR(100), 
    //         published DATETIME, 
    //         summary VARCHAR(500), 
    //         length INTEGER, 
    //         audio INTEGER, 
    //         href VARCHAR(250), 
    //         downloaded INTEGER, 
    //         podcast_id INTEGER, veiwed Integer, 
    //         PRIMARY KEY (episode_id), 
    //         FOREIGN KEY(podcast_id) REFERENCES podcasts (podcast_id)
    //     )",
    //     [],
    // )?;
    // let me = Category {
    //     id: 0,
    //     name: "News".to_string(),
    // };
    // let temp = conn.execute(
    //     "INSERT INTO category (name) VALUES (?1)",
    //     params![me.name],
    // )?;


    // let me2 = Category {
    //     id: 0,
    //     name: "Rock".to_string(),
    // };
    // let temp = conn.execute(
    //     "INSERT INTO category (name) VALUES (?1)",
    //     params![me2.name],
    // )?;

    // println!("{}", temp);

    // let mut stmt = conn.prepare("SELECT id, name FROM category")?;
    // let cat_iter = stmt.query_map([], |row| {
    //     Ok(Category {
    //         id: row.get(0)?,
    //         name: row.get(1)?,
    //     })
    // })?;

    // for category in cat_iter {
    //     println!("Found {:?}", category.unwrap());
    // }
    Ok(())
}