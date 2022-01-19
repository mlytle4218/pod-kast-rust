use rusqlite::{params, Connection, Error, Result};

#[derive(Debug)]
struct Category {
    id: i32,
    name: String,
}
impl Category {
    pub fn new() -> Category {
        Category {
            id: 0,
            name: String::from("nada"),
        }
    }
    fn create_category(&self, conn: Connection) -> Result<usize, Error> {
        let result = conn.execute(
            "INSERT INTO categories (name) VALUES (?1)",
            params![self.name],
        )?;
        Ok(result)
    }
    fn read_categories(&self, conn: Connection) -> Result<Vec<Category>, Error> {
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

    fn update_category(&self, conn: Connection) -> Result<usize, Error> {
        let result = conn.execute(
            "UPDATE categories SET name=(?1) where category_id=(?2)",
            params![self.name, self.id],
        )?;
        Ok(result)
    }

    fn delete_category(&self, conn: Connection) -> Result<usize, Error> {
        let result = conn.execute("DELETE FROM categories where category_id=(?1)", params![self.id])?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct LocalTestContext {
        pub conn: Connection,
    }
    impl LocalTestContext {
        fn new() -> LocalTestContext {
            let _conn = TestContext::new().conn;
            _conn.execute(
                "INSERT INTO categories (name) VALUES (?1)",
                params![String::from("News")],
            ).unwrap();
            LocalTestContext { conn: _conn }
        }
    }
    use super::super::context::TestContext;
    use super::*;

    #[test]
    fn test_create_category() {
        let _conn = LocalTestContext::new();
        let cat = Category {
            id: 0,
            name: String::from("Rock Music"),
        };
        let res = cat.create_category(_conn.conn).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_read_category() {
        let _conn = LocalTestContext::new();
        let cat = Category::new();
        let _res = cat.read_categories(_conn.conn).unwrap();
        assert_eq!(_res.len(), 1);
        assert_eq!(_res[0].id, 1);
    }

    #[test]
    fn test_update_category_by_id() {
        let _conn = LocalTestContext::new();
        let cat = Category {
            id: 1,
            name: String::from("Rock Music"),
        };
        let res = cat.update_category(_conn.conn).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_delete_category_by_id() {
        let _conn = LocalTestContext::new();
        let cat = Category {
            id: 1,
            name: String::from("Rock Music"),
        };
        let res = cat.delete_category(_conn.conn).unwrap();
        assert_eq!(res, 1);
    }
}
