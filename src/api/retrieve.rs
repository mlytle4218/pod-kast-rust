use chrono::Utc;
use rss::Channel;
use std::error::Error;

use crate::data::episode::Episode;
use log::error;

pub struct Retreive {

}

impl Retreive {
    pub fn new() -> Retreive {
        Retreive {}
    }

    #[tokio::main]
    pub async fn retreive_episodes(&self, url: String, podcast_id: i16) -> Result<Vec<Episode>, Box<dyn Error>> {
        let content = reqwest::get(url).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;

        let mut episode_vec: Vec<Episode> = Vec::new();
        for it in channel.items() {
            
            let en: &rss::Enclosure = it.enclosure.as_ref().unwrap();

            match String::from(&en.length).parse::<i32>() {
                Ok(temp_length) =>{
                    episode_vec.push(Episode {
                        id: 0,
                        title: String::from(it.title.as_ref().unwrap()),
                        published: Utc::now(),
                        summary: String::from(it.description.as_ref().unwrap()),
                        length: temp_length,
                        audio: String::from(&en.mime_type),
                        url: String::from(&en.url),
                        downloaded: 0,
                        viewed: 0,
                        podcast_id: podcast_id,
                        queue: 0
                    });
                },
                Err(e) => {
                    error!("{}", e)
                }
            }

            
            // episode_vec.push(Episode {
            //     id: 0,
            //     title: String::from(it.title.as_ref().unwrap()),
            //     published: Utc::now(),
            //     summary: String::from(it.description.as_ref().unwrap()),
            //     length: String::from(&en.length).parse::<i32>().unwrap(),
            //     audio: String::from(&en.mime_type),
            //     url: String::from(&en.url),
            //     downloaded: 0,
            //     viewed: 0,
            //     podcast_id: podcast_id,
            // });
        }
        Ok(episode_vec)
    }

}