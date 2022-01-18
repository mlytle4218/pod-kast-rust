use rusqlite::{params, Connection, Error, Result};


#[derive(Debug)]
struct Category {
    id: i32,
    name: String,
}
impl Category  {
    fn create_category(&self, conn: Connection) -> Result<usize, Error> {
    let result = conn.execute("INSERT INTO categories (name) VALUES (?1)", params![self.name])?;
    Ok(result)
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
    }
    Ok(results)
}

fn update_category_by_id(conn: Connection, cat: Category) -> Result<usize, Error> {
    let result = conn.execute("UPDATE categories SET name=(?1) where id=(?2)", params![cat.name, cat.id])?;
    Ok(result)
}

fn delete_category_by_id(conn: Connection, cat: Category) -> Result<usize, Error> {
    let result = conn.execute("DELETE FROM categories where id=(?1)", params![cat.id])?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::context::TestContext;

    #[test]
    fn test_create_category() {
        let _conn = TestContext::new();
        let cat = Category {
            id: 0,
            name: String::from("Rock Music")
        };
        
        let res = cat.create_category(_conn.conn).unwrap();
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
        let cat = Category {
            id: 1,
            name: String::from("Rock Music")
        };
        let res = update_category_by_id(_conn.conn, cat).unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn test_delete_category_by_id() {
        let _conn = TestContext::new();
        let cat = Category {
            id: 1,
            name: String::from("Rock Music")
        };
        let res = delete_category_by_id(_conn.conn, cat).unwrap();
        assert_eq!(res, 1);
    }
}
