use termsize;

use super::screen::Screen;
use super::super::data::podcast::Podcast;
use super::menu_entry::MenuEntry;

pub struct PodcastMenu {
    screen: Screen
}

impl PodcastMenu {
    pub fn new(screen: Screen) -> PodcastMenu {
        PodcastMenu {
            screen: screen
        }
    }
    pub fn show_podcasts(&self, podcasts: Vec<Podcast>) -> Vec<Podcast> {
        let result: Vec<Podcast> = Vec::new();
        result
    }
}