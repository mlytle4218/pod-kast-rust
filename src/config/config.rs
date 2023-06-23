use serde::{Serialize, Deserialize};
use std::fs;
use whoami;
use toml;

use log::{info,error};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub asset_location: String,
    config_file: String,
    pub def_audio_loc: String,
    pub def_video_loc: String,
    pub database: Database
}
#[derive(Serialize, Deserialize, Debug, Clone)]
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
        let con = Config {
            asset_location: format!("/home/{}/.pod-kast/", whoami::username()),
            config_file: String::from("pod-kast-config"),
            database: db,
            def_audio_loc: format!("/home/{}/audio/", whoami::username()),
            def_video_loc: format!("/home/{}/video/", whoami::username())
        };
        match con.load_config() {
            Ok(result) =>{
                return result
            },
            Err(_) =>{
                match fs::create_dir_all(&con.asset_location){
                    Ok(_)  =>{
                        // Config::save_config(&con).unwrap();
                        match Config::save_config(&con) {
                            Ok(_) =>{
                                info!("{} saved", con.config_file);
                            },
                            Err(e) => {error!("load_config error: {}",e);}
                        }
                    },
                    Err(e) => {error!("load_config error: {}",e);}
                }
            }
        }
        con
    }
    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let res = format!("{}/{}", self.asset_location, self.config_file);
        match toml::to_string(self) {
            Ok(toml) => {
                match fs::write(res, toml) {
                    Ok(_) => {
                        return Ok(())
                    }, Err(e) =>{
                        error!("save_config-fs:write: {}",e);
                        return Err(Box::new(e))
                    }
                }

            }, Err(e) => {
                error!("save_config-toml::to_string: {}", e);
                return Err(Box::new(e))
            }
        }
    }
    fn load_config(&self) -> Result<Config, Box<dyn std::error::Error> > {
        let res = format!("{}/{}", self.asset_location, self.config_file);
        match fs::read_to_string(res) {
            Ok(data) =>{
                match toml::from_str(&data) {
                    Ok(config) =>{
                        return Ok(config)
                    },
                    Err(e) => return Err(Box::new(e))
                }

            },
            Err(e) => return Err(Box::new(e))
        }
    }
}
