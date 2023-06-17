use rusqlite::{params, Connection, Error, Result};

use log::info;

use super::super::config::config::Config;
use super::data::DB;

#[derive(Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
}
impl Category {
    pub fn new() -> Category {
        Category {
            id: 0,
            name: String::from("nada"),
        }
    }
    pub fn create_exisitng(&self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        info!("create_catrgory");
        let result = conn.execute(
            "INSERT INTO categories (category) VALUES (?1)",
            params![self.name],
        )?;
        Ok(result)
    }
    pub fn update_existing(&self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let result = conn.execute(
            "UPDATE categories SET category=(?1) where category_id=(?2)",
            params![self.name, self.id],
        )?;
        Ok(result)
    }
    pub fn delete_existing(&self) -> Result<usize, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
        let result = conn.execute("DELETE FROM categories where category_id=(?1)", params![self.id])?;
        Ok(result)
    }
    pub fn read_all_categories(&self) -> Result<Vec<Category>, Error> {
        let db: DB = DB::new(Config::new());
        let conn: Connection = db.connect_to_database();
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




    // pub fn to_string(&self) -> String {
    //     format!("category name and id {} {}",self.name, self.id)
    // }
    // pub fn create_category(&self, conn: Connection) -> Result<usize, Error> {
    //     info!("create_catrgory");
    //     let result = conn.execute(
    //         "INSERT INTO categories (category) VALUES (?1)",
    //         params![self.name],
    //     )?;
    //     Ok(result)
    // }
    // // pub fn read_categories(&self, conn: &Connection) -> Result<Vec<Category>, ReadlineError> {
    // pub fn read_categories(&self, conn: &Connection) -> Result<Vec<Category>, Error> {
    //     let mut stmt = conn.prepare("SELECT * FROM categories;")?;
    //     let cat_iter = stmt.query_map([], |row| {
    //         Ok(Category {
    //             id: row.get(0)?,
    //             name: row.get(1)?,
    //         })
    //     })?;
    //     let mut results: Vec<Category> = Vec::new();
    //     for category in cat_iter {
    //         results.push(category.unwrap());
    //     }
    //     Ok(results)
    // }
    // pub fn read_category_by_id(&self, conn: &Connection, id: usize) -> Result<Category, Error> {
    //     // pub fn read_category_by_id(&self, conn: &Connection, id: usize) -> Result<Vec<Category>, Error> {
    //     let mut result = conn.prepare(
    //         "SELECT * FROM categories where category_id=:id"
    //     )?;
    //     let cat_iter = result.query_map(&[(":id", &id)], |row| {
    //         Ok(Category {
    //             id: row.get(0)?,
    //             name: row.get(1)?,
    //         })
    //     })?;
    //     let mut results: Category = Category::new();
    //     for category in cat_iter {
    //         let temp = category.unwrap();
    //         results.id = temp.id;
    //         results.name =  temp.name;
    //     }
    //     Ok(results)

    // }
    // pub fn update_category(&self, conn: &Connection) -> Result<usize, Error> {
    //     let result = conn.execute(
    //         "UPDATE categories SET category=(?1) where category_id=(?2)",
    //         params![self.name, self.id],
    //     )?;
    //     Ok(result)
    // }
    // pub fn delete_category(&self, conn: Connection) -> Result<usize, Error> {
    //     let result = conn.execute("DELETE FROM categories where category_id=(?1)", params![self.id])?;
    //     Ok(result)
    // }


}

impl Clone for Category {
    fn clone(&self) -> Category {
        Category {
            id: self.id.clone(),
            name: self.name.clone()
        }
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
