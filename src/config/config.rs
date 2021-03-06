use serde::{Serialize, Deserialize};
use std::fs;
use whoami;
use toml;


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub asset_location: String,
    config_file: String,
    pub database: Database,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub sqlite_file: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
}
impl Config {

    pub fn new() -> Config {
        let db = Database {
            sqlite_file: String::from("pod-kast.db")
        };
        let mut con = Config {
            asset_location: format!("/home/{}/.pod-kast/", whoami::username()),
            config_file: String::from("pod-kast-config"),
            database: db,
        };
        if con.config_exists() {
            con = con.load_config();
        } else {
            match fs::create_dir_all(&con.asset_location){
                Ok(_)  =>{Config::save_config(&con).unwrap();},
                Err(_) => {}
            }            
        };
        con
    }
    fn config_exists(&self) -> bool {
        // let user: String = whoami::username();
        let res = format!("{}/{}", self.asset_location, self.config_file);
        fs::metadata(res).is_ok()
    }
    pub fn save_config(&self) -> std::io::Result<()> {
        let toml = toml::to_string(self).unwrap();
        let res = format!("{}/{}", self.asset_location, self.config_file);
        fs::write(res, toml)?;
        Ok(())
    }


    fn load_config(&self) -> Config {
        let res = format!("{}/{}", self.asset_location, self.config_file);
        let data = fs::read_to_string(res).unwrap();
        let config: Config = toml::from_str(&data).unwrap();
        config
    }
}
