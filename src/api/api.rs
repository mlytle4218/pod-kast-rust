// use reqwest;
// use serde_json::{Value};

// use crate::data::podcast;


// fn build_request(terms: &str, url: Option<String>) -> String {
//     let number_of_search_results: i8 = 2;
//     let result: String;
//     if url.is_none() {
//         result = format!("https://itunes.apple.com/search?term={}&entity=podcast&limit={}", terms, number_of_search_results);
//     } else {
//         result = url.unwrap();
//     }
//     result
// }

// #[tokio::main]
// pub async fn search(_terms: &str) -> Result<Value, Box<dyn std::error::Error>> {
//     let client = reqwest::Client::builder()
//         .build()?;

//     let url = build_request(_terms, None);

//     let res = client
//         .get(url)
//         .send()
//         .await?
//         .text()
//         .await?;

//     let p: Value = serde_json::from_str(&res[..])?;

//     // println!("{}", p["results"][0]);

//     Ok(p)
// }


// #[cfg(test)]
// mod test {
//     use super::*;

//     #[tokio::main]
//     async fn test_main() {
//         let res = search("news");
//         assert_eq!(res, true);
//     }
// }

// {
// 	"resultCount": 2,
// 	"results": [{
// 			"wrapperType": "track",
// 			"kind": "podcast",
// 			"artistId": 121676617,
// 			"collectionId": 135067274,
// 			"trackId": 135067274,
// 			"artistName": "BBC World Service",
// 			"collectionName": "Global News Podcast",
// 			"trackName": "Global News Podcast",
// 			"collectionCensoredName": "Global News Podcast",
// 			"trackCensoredName": "Global News Podcast",
// 			"artistViewUrl": "https://podcasts.apple.com/us/artist/itunesu-sunset/121676617?uo=4",
// 			"collectionViewUrl": "https://podcasts.apple.com/us/podcast/global-news-podcast/id135067274?uo=4",
// 			"feedUrl": "https://podcasts.files.bbci.co.uk/p02nq0gn.rss",
// 			"trackViewUrl": "https://podcasts.apple.com/us/podcast/global-news-podcast/id135067274?uo=4",
// 			"artworkUrl30": "https://is1-ssl.mzstatic.com/image/thumb/Podcasts115/v4/23/9d/ce/239dcef7-d24d-bb0b-1ed1-7b7a87d635ce/mza_7839052379663357293.jpg/30x30bb.jpg",
// 			"artworkUrl60": "https://is1-ssl.mzstatic.com/image/thumb/Podcasts115/v4/23/9d/ce/239dcef7-d24d-bb0b-1ed1-7b7a87d635ce/mza_7839052379663357293.jpg/60x60bb.jpg",
// 			"artworkUrl100": "https://is1-ssl.mzstatic.com/image/thumb/Podcasts115/v4/23/9d/ce/239dcef7-d24d-bb0b-1ed1-7b7a87d635ce/mza_7839052379663357293.jpg/100x100bb.jpg",
// 			"collectionPrice": 0.00,
// 			"trackPrice": 0.00,
// 			"trackRentalPrice": 0,
// 			"collectionHdPrice": 0,
// 			"trackHdPrice": 0,
// 			"trackHdRentalPrice": 0,
// 			"releaseDate": "2022-01-12T22:21:00Z",
// 			"collectionExplicitness": "cleaned",
// 			"trackExplicitness": "cleaned",
// 			"trackCount": 57,
// 			"country": "USA",
// 			"currency": "USD",
// 			"primaryGenreName": "News",
// 			"contentAdvisoryRating": "Clean",
// 			"artworkUrl600": "https://is1-ssl.mzstatic.com/image/thumb/Podcasts115/v4/23/9d/ce/239dcef7-d24d-bb0b-1ed1-7b7a87d635ce/mza_7839052379663357293.jpg/600x600bb.jpg",
// 			"genreIds": ["1489", "26"],
// 			"genres": ["News", "Podcasts"]
// 		}
// 	]
// }