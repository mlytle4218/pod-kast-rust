use rusqlite::{ Connection, Error };
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

// #[derive(Debug)]
// struct TestContext {
//     conn: String
// }

// impl TestContext {
//     fn new() -> TestContext {
//         TestContext {
//             conn: String::from("hiya")
//         }
//     }
// }

// impl Drop for TestContext {
//     fn drop(&mut self) {
//         println!("clean up resources");
//     }
// }

#[test]
fn try_it() {
    let conn = Connection::open(":memory:").unwrap();
    
    get_categories(conn);
}