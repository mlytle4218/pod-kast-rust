use rusqlite::{Connection};
use std::fs;

pub struct DB {
    pub sqlite_file_location: String
}

use super::super::config::config::Config;

impl DB {
    pub fn new(config: Config) -> DB {
        let url = format!("{}/{}", config.asset_location, config.database.sqlite_file);
        DB {
            sqlite_file_location: url 
        }
    }

    pub fn connect_to_database(&self) -> Connection {
        if fs::metadata(&self.sqlite_file_location).is_ok() {
            let conn = Connection::open(&self.sqlite_file_location).unwrap();
            conn
        } else {
            let conn = Connection::open(&self.sqlite_file_location).unwrap();
            conn.execute("
                CREATE TABLE categories (
                    category_id INTEGER NOT NULL, 
                    category VARCHAR(250), 
                    PRIMARY KEY (category_id)
                );
            ", []).unwrap();
            conn.execute("
                CREATE TABLE podcasts (
                    podcast_id INTEGER NOT NULL, 
                    name VARCHAR(250), 
                    url VARCHAR(250), 
                    audio VARCHAR(250), 
                    video VARCHAR(250),
                    category_id INTEGER,
                    collection_id INTEGER,
                    viewed INTEGER,
                    PRIMARY KEY (podcast_id), 
                    FOREIGN KEY(category_id) REFERENCES categories (category_id),
                    UNIQUE(name)
                );
            ", []).unwrap();
            conn.execute("
                CREATE TABLE episodes (
                    episode_id INTEGER NOT NULL, 
                    title VARCHAR(100), 
                    published DATETIME, 
                    summary VARCHAR(500), 
                    length INTEGER, 
                    audio INTEGER, 
                    url VARCHAR(250), 
                    downloaded INTEGER, 
                    podcast_id INTEGER, 
                    viewed INTEGER, 
                    PRIMARY KEY (episode_id), 
                    FOREIGN KEY(podcast_id) REFERENCES podcasts (podcast_id),
                    UNIQUE(url, title)
                );
            ", []).unwrap();
            conn.execute("INSERT INTO categories (category) VALUES ('Uncategorized');", []).unwrap();
            conn
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DB;
    use crate::data::category::Category;
    use rusqlite::{Connection, NO_PARAMS};

    #[test]
    fn test_database() {
        
        let db = DB {
            sqlite_file_location: String::from(":memory:")
        };

        let conn = db.connect_to_database();
        let mut stmt = conn.prepare(
            "SELECT * from categories;",
        ).unwrap();
    
        let cats = stmt.query_map(NO_PARAMS, |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        }).unwrap();

        let mut cat_vec: Vec<Category> = Vec::new();
    
        for cat in cats {
            cat_vec.push(cat.unwrap());
        }

        assert_eq!(cat_vec[0].name, String::from("Uncategorized"));
    }
}