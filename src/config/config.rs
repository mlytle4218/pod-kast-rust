use serde::{Serialize, Deserialize};
use std::fs;
use whoami;
use toml;

use log::{info,error};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub sqlite_file: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LogInfo {
    pub logging_file_path: String,
    pub logging_type: String
}
impl Clone for LogInfo {
    fn clone(&self) ->  LogInfo {
        LogInfo {
            logging_file_path: self.logging_file_path.clone(),
            logging_type: self.logging_type.clone()
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PathInfo {
    pub asset_location: String,
    pub config_file: String,
    pub def_audio_loc: String,
    pub def_video_loc: String,
}
impl Clone for PathInfo {
    fn clone(&self) -> PathInfo {
        PathInfo {
            asset_location: self.asset_location.clone(),
            config_file: self.config_file.clone(),
            def_audio_loc: self.def_audio_loc.clone(),
            def_video_loc: self.def_video_loc.clone(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Specs {
    pub separator: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub database: Database,
    pub pi: PathInfo,
    pub log_info: LogInfo,
    pub specs: Specs
}
impl Config {
    pub fn new() -> Config {
        let db = Database {
            sqlite_file: String::from("pod-kast.db")
        };
        let p_i = PathInfo {
            asset_location: format!("/home/{}/.pod-kast/", whoami::username()),
            config_file: String::from("pod-kast-config"),
            def_audio_loc: format!("/home/{}/audio/", whoami::username()),
            def_video_loc: format!("/home/{}/video/", whoami::username()),
        };
        let l_i = LogInfo {
            logging_file_path: format!("/home/{}/logging_config.yaml", whoami::username()),
            logging_type: format!("syslog")
        };
        let specs = Specs {
            separator: " ".to_string()
        };
        let con = Config {
            database: db,
            pi: p_i,
            log_info: l_i,
            specs: specs

        };
        match con.load_config() {
            Ok(result) =>{
                return result
            },
            Err(_) =>{
                match fs::create_dir_all(&con.pi.asset_location){
                    Ok(_)  =>{
                        // Config::save_config(&con).unwrap();
                        match Config::save_config(&con) {
                            Ok(_) =>{
                                info!("{} saved", con.pi.config_file);
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
        let res = format!("{}/{}", self.pi.asset_location, self.pi.config_file);
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
        let res = format!("{}/{}", self.pi.asset_location, self.pi.config_file);
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
