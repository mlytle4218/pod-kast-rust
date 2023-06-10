use super::screen::Screen;
// use super::super::data::podcast::Podcast;

pub struct PodcastMenu {
    screen: Screen
}

impl PodcastMenu {
    pub fn new(screen: Screen) -> PodcastMenu {
        PodcastMenu {
            screen: screen
        }
    }
    // pub fn show_podcasts(&self, _podcasts: Vec<Podcast>) -> Vec<Podcast> {
    //     let result: Vec<Podcast> = Vec::new();
    //     result
    // }
}