use rusqlite::{params, Connection, Error, Result};

#[derive(Debug)]
struct Category {
    id: i32,
    name: String,
}

fn create_category(conn: Connection, name: String) -> Result<usize, Error> {
    let result = conn.execute("INSERT INTO categories (name) VALUES (?1)", params![name])?;
    Ok(result)
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
    }
    Ok(results)
}

fn update_category_by_id(conn: Connection, id: i8, name: String) -> Result<usize, Error> {
    let result = conn.execute("UPDATE categories SET name=(?1) where id=(?2)", params![name, id])?;
    Ok(result)
}

fn delete_category_by_id(conn: Connection, id: i8) -> Result<usize, Error> {
    let result = conn.execute("DELETE FROM categories where id=(?1)", params![id])?;
    Ok(result)
}

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
                "INSERT INTO categories (name) VALUES (?1)",
                params![String::from("News")],
            )?;
            Ok(_conn)
        }
        fn new() -> TestContext {
            let _conn = TestContext::create().unwrap();
            TestContext { conn: _conn }
        }
    }

    #[test]
    fn test_create_category() {
        let _conn = TestContext::new();
        let res = create_category(_conn.conn, String::from("Rock")).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_read_category() {
        let _conn = TestContext::new();
        let _res = read_categories(_conn.conn).unwrap();
        assert_eq!(_res.len(), 1);
        assert_eq!(_res[0].id, 1);
    }

    #[test]
    fn test_update_category_by_id() {
        let _conn = TestContext::new();
        let res = update_category_by_id(_conn.conn, 1, String::from("News-new")).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_delete_category_by_id() {
        let _conn = TestContext::new();
        let res = delete_category_by_id(_conn.conn, 1).unwrap();
        assert_eq!(res, 1);
    }
}
