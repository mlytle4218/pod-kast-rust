use reqwest;
use serde::{Deserialize};

use crate::data::podcast::Podcast;


#[derive(Deserialize, Debug)]
pub struct ItuneResult {
	resultCount: i32,
	results: Vec<PodcastResult>,
}
#[derive(Deserialize, Debug)]
struct PodcastResult {
	collectionName: String,
	feedUrl: String,
}

fn build_request(terms: &str, url: Option<String>) -> String {
	let number_of_search_results: i8 = 2;
	let result: String;
	if url.is_none() {
		result = format!(
			"https://itunes.apple.com/search?term={}&entity=podcast&limit={}",
			terms, number_of_search_results
		);
	} else {
		result = url.unwrap();
	}
	result
}

#[tokio::main]
pub async fn search(_terms: &str) -> Result<Vec<Podcast>, Box<dyn std::error::Error>> {
	let client = reqwest::Client::builder().build()?;

	let url = build_request(_terms, None);

	let res = client.get(url).send().await?;

	let itune_results = res.json::<ItuneResult>().await?;
	
	let mut podcast_vec: Vec<Podcast> = Vec::new();

	for itune_result in itune_results.results {
		let temp = Podcast {
			id: 0,
			name: itune_result.collectionName,
			url: itune_result.feedUrl,
			audio: String::from("/home/marc/audio"),
			video: String::from("/home/marc/video"),
			category_id: 1
		};
		podcast_vec.push(temp);
	}

	Ok(podcast_vec)
}

#[cfg(test)]
mod test {
	use super::*;

	#[tokio::main]
	async fn test_main() {
		let res = search("news");
		assert_eq!(true, true);
	}
}

// {
// 	resultCount: 2,
// 	results: [{
// 			wrapperType: track,
// 			kind: podcast,
// 			artistId: 121676617,
// 			collectionId: 135067274,
// 			trackId: 135067274,
// 			artistName: BBC World Service,
// 			collectionName: Global News Podcast,
// 			trackName: Global News Podcast,
// 			collectionCensoredName: Global News Podcast,
// 			trackCensoredName: Global News Podcast,
// 			artistViewUrl: String,
// 			collectionViewUrl: String,
// 			feedUrl: String,
// 			trackViewUrl: String,
// 			artworkUrl30: String,
// 			artworkUrl60: String,
// 			artworkUrl100: String,
// 			collectionPrice: f32,
// 			trackPrice: f32,
// 			trackRentalPrice: i32,
// 			collectionHdPrice: i32,
// 			trackHdPrice: i32,
// 			trackHdRentalPrice: i32,
// 			releaseDate: 2022-01-12T22:21:00Z,
// 			collectionExplicitness: cleaned,
// 			trackExplicitness: cleaned,
// 			trackCount: 57,
// 			country: USA,
// 			currency: USD,
// 			primaryGenreName: News,
// 			contentAdvisoryRating: Clean,
// 			artworkUrl600: String,
// 			genreIds: [1489, 26],
// 			genres: [News, Podcasts]
// 		}
// 	]
// }
