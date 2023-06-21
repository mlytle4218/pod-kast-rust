use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use crate::data::podcast::Podcast;
use super::super::config::config::Config;
use log::info;

#[derive(Deserialize)]
pub struct ItuneResult {
	results: Vec<PodcastResult>,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct PodcastResult {
	collectionName: String,
	feedUrl: String,
	collectionId: i32
}

pub struct AppleSearch {
	base_url: String,
	terms: String,
	limit: u8,
}

impl AppleSearch {
	pub fn new(base_url: String, terms: String, limit: u8) -> AppleSearch {
		let re = Regex::new(r"\s+").unwrap();
		let terms_prepped = String::from(re.replace_all(&terms[..], "+"));
		AppleSearch {
			base_url: base_url,
			terms: terms_prepped,
			limit: limit,
		}
	}

	#[tokio::main]
	pub async fn search(&self) -> Result<Vec<Podcast>, Box<dyn std::error::Error>> {
		info!("in search *****************************");

		let config: Config = Config::new();

		let url = format!(
			"{}/search?term={}&entity=podcast&limit={}",
			self.base_url, self.terms, self.limit
		);
		let client = Client::builder().build()?;
		
		info!("in search 2*****************************");

		let res = client.get(url).send().await?;
		info!("in search 3*****************************");
		info!("{:?}", res);

		let itune_results = res.json::<ItuneResult>().await?;
		info!("in search 4*****************************");
		let mut podcast_vec: Vec<Podcast> = Vec::new();
		info!("in search 5*****************************");

		for itune_result in itune_results.results {
			let temp = Podcast {
				id: 0,
				name: itune_result.collectionName,
				url: itune_result.feedUrl,
				audio: config.def_audio_loc.clone(),
				video: config.def_video_loc.clone(),
				category_id: -1,
				collection_id: itune_result.collectionId
			};
			podcast_vec.push(temp);
		}
		info!("number of podcasts returned: {}", podcast_vec.len());
		Ok(podcast_vec)
	}
}





// #[tokio::main]
// pub async fn retreive_episodes(url: String, podcast_id: i16) -> Result<Vec<Episode>, Box<dyn Error>> {
// 	let content = reqwest::get(url).await?.bytes().await?;
// 	let channel = Channel::read_from(&content[..])?;

// 	let mut episode_vec: Vec<Episode> = Vec::new();
// 	for it in channel.items() {
		
// 		let en: &rss::Enclosure = it.enclosure.as_ref().unwrap();
		
// 		episode_vec.push(Episode {
// 			id: 0,
// 			title: String::from(it.title.as_ref().unwrap()),
// 			published: Utc::now(),
// 			summary: String::from(it.description.as_ref().unwrap()),
// 			length: String::from(&en.length).parse::<i32>().unwrap(),
// 			audio: String::from(&en.mime_type),
// 			url: String::from(&en.url),
// 			viewed: 0,
// 			downloaded: 0,
// 			podcast_id: podcast_id,
// 			queue: 0
// 		});
// 	}
// 	Ok(episode_vec)
// }

#[cfg(test)]
mod test {
	use super::*;

	use super::AppleSearch;
	use httpmock::MockServer;
	use serde_json::json;

	#[test]
	fn test_apple_search() {
		let server = MockServer::start();

		let search = AppleSearch::new(server.base_url(), String::from("News"), 2);

		let mock = server.mock(|when, then| {
			when.method("GET")
				.path("/search")
				.query_param("term", &search.terms)
				.query_param("entity", "podcast")
				.query_param("limit", &search.limit.to_string());
			then.status(200).json_body(json!({
				"resultCount": 2,
				"results": [
					{"wrapperType":"track", "kind":"podcast", "artistId":121676617, "collectionId":135067274, "trackId":135067274, "artistName":"BBC World Service", "collectionName":"Global News Podcast", "trackName":"Global News Podcast", "collectionCensoredName":"Global News Podcast", "trackCensoredName":"Global News Podcast", "artistViewUrl":"https://podcasts.apple.com/us/artist/bbc/121676617?uo=4", "collectionViewUrl":"https://podcasts.apple.com/us/podcast/global-news-podcast/id135067274?uo=4", "feedUrl":"https://podcasts.files.bbci.co.uk/p02nq0gn.rss", "trackViewUrl":"https://podcasts.apple.com/us/podcast/global-news-podcast/id135067274?uo=4", "artworkUrl30":"https://is1-ssl.mzstatic.com/image/thumb/Podcasts115/v4/23/9d/ce/239dcef7-d24d-bb0b-1ed1-7b7a87d635ce/mza_7839052379663357293.jpg/30x30bb.jpg", "artworkUrl60":"https://is1-ssl.mzstatic.com/image/thumb/Podcasts115/v4/23/9d/ce/239dcef7-d24d-bb0b-1ed1-7b7a87d635ce/mza_7839052379663357293.jpg/60x60bb.jpg", "artworkUrl100":"https://is1-ssl.mzstatic.com/image/thumb/Podcasts115/v4/23/9d/ce/239dcef7-d24d-bb0b-1ed1-7b7a87d635ce/mza_7839052379663357293.jpg/100x100bb.jpg", "collectionPrice":0.00, "trackPrice":0.00, "trackRentalPrice":0, "collectionHdPrice":0, "trackHdPrice":0, "trackHdRentalPrice":0, "releaseDate":"2022-01-21T15:49:00Z", "collectionExplicitness":"cleaned", "trackExplicitness":"cleaned", "trackCount":56, "country":"USA", "currency":"USD", "primaryGenreName":"News", "contentAdvisoryRating":"Clean", "artworkUrl600":"https://is1-ssl.mzstatic.com/image/thumb/Podcasts115/v4/23/9d/ce/239dcef7-d24d-bb0b-1ed1-7b7a87d635ce/mza_7839052379663357293.jpg/600x600bb.jpg", "genreIds":["1489", "26"], "genres":["News", "Podcasts"]}, 
					{"wrapperType":"track", "kind":"podcast", "collectionId":1027264941, "trackId":1027264941, "artistName":"Airwave Media", "collectionName":"Kickass News", "trackName":"Kickass News", "collectionCensoredName":"Kickass News", "trackCensoredName":"Kickass News", "collectionViewUrl":"https://podcasts.apple.com/us/podcast/kickass-news/id1027264941?uo=4", "feedUrl":"https://kickasspoliticspreview1.libsyn.com/kickasspolitics2", "trackViewUrl":"https://podcasts.apple.com/us/podcast/kickass-news/id1027264941?uo=4", "artworkUrl30":"https://is3-ssl.mzstatic.com/image/thumb/Podcasts123/v4/2d/90/aa/2d90aa4a-42ba-2913-49cc-5b5889ca1546/mza_11701255775750433427.jpg/30x30bb.jpg", "artworkUrl60":"https://is3-ssl.mzstatic.com/image/thumb/Podcasts123/v4/2d/90/aa/2d90aa4a-42ba-2913-49cc-5b5889ca1546/mza_11701255775750433427.jpg/60x60bb.jpg", "artworkUrl100":"https://is3-ssl.mzstatic.com/image/thumb/Podcasts123/v4/2d/90/aa/2d90aa4a-42ba-2913-49cc-5b5889ca1546/mza_11701255775750433427.jpg/100x100bb.jpg", "collectionPrice":0.00, "trackPrice":0.00, "trackRentalPrice":0, "collectionHdPrice":0, "trackHdPrice":0, "trackHdRentalPrice":0, "releaseDate":"2022-01-20T19:01:00Z", "collectionExplicitness":"cleaned", "trackExplicitness":"cleaned", "trackCount":590, "country":"USA", "currency":"USD", "primaryGenreName":"Society & Culture", "contentAdvisoryRating":"Clean", "artworkUrl600":"https://is3-ssl.mzstatic.com/image/thumb/Podcasts123/v4/2d/90/aa/2d90aa4a-42ba-2913-49cc-5b5889ca1546/mza_11701255775750433427.jpg/600x600bb.jpg", "genreIds":["1324", "26", "1489"], "genres":["Society & Culture", "Podcasts", "News"]}
				]
			}));
		});

		// let search = AppleSearch::new(server.base_url(), String::from("News"), 2);

		let res: Vec<Podcast> = search.search().unwrap();
		assert_eq!(res[0].name, String::from("Global News Podcast"));
	}

	#[test]
	fn test_retreive_episodes() {
		let server = MockServer::start();

		let mock = server.mock(|when, then| {
			when.method("GET");
			then.status(200)
			.header("Content-Type", "text/xml; charset=UTF-8")
			.body(r#"<?xml version="1.0" encoding="UTF-8"?><rss version="2.0" xmlns:media="http://search.yahoo.com/mrss/" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:ppg="http://bbc.co.uk/2009/01/ppgRss" xmlns:itunes="http://www.itunes.com/dtds/podcast-1.0.dtd"><!--By accessing this feed you are warranting that you are using these feeds in accordance with the BBCs Terms of Use http://www.bbc.co.uk/usingthebbc/terms-of-use/. If you are using these feeds for business or commercial purposes you warrant that you have obtained the necessary licence from the BBC and that BBC services and content are made available strictly in line with the BBCs Distribution Policy--><channel><title>Global News Podcast</title><link>http://www.bbc.co.uk/programmes/p02nq0gn</link><description>The day’s top stories from BBC News. Delivered twice a day on weekdays, daily at weekends</description><itunes:summary>The day’s top stories from BBC News. Delivered twice a day on weekdays, daily at weekends</itunes:summary><itunes:author>BBC World Service</itunes:author><itunes:owner><itunes:name>BBC</itunes:name><itunes:email>RadioMusic.Support@bbc.co.uk</itunes:email></itunes:owner><itunes:new-feed-url>https://podcasts.files.bbci.co.uk/p02nq0gn.rss</itunes:new-feed-url><language>en</language><ppg:seriesDetails frequency="daily" daysLive="30"/><ppg:systemRef systemId="pid.brand" key="p02nq0gn"/><ppg:systemRef systemId="pid.genre" key="C00079"/><ppg:network id="worldserviceradio" name="BBC World Service"/><image><url>http://ichef.bbci.co.uk/images/ic/3000x3000/p09kz1gg.jpg</url><title>Global News Podcast</title><link>http://www.bbc.co.uk/programmes/p02nq0gn</link></image><itunes:image href="http://ichef.bbci.co.uk/images/ic/3000x3000/p09kz1gg.jpg"/><copyright>(C) BBC 2022</copyright><pubDate>Thu, 20 Jan 2022 15:39:00 +0000</pubDate><itunes:category text="News"/><itunes:explicit>no</itunes:explicit><media:rating scheme="urn:simple">nonadult</media:rating><atom:link href="http://www.bbc.co.uk/programmes/p02nq0gn/episodes/downloads.rss" rel="self" type="application/rss+xml"/><item><title>Former Pope Benedict accused of failing to act over child abuse cases</title><description>Report claims he failed to act to prevent abuse when he was Archbishop of Munich. Also, the ICRC is targeted by cyber hackers and, the first aid flights arrive in Tonga.</description><itunes:subtitle>Report claims he failed to act to prevent abuse when he was Archbishop of Munich</itunes:subtitle><itunes:summary>Report claims he failed to act to prevent abuse when he was Archbishop of Munich. Also, the ICRC is targeted by cyber hackers and, the first aid flights arrive in Tonga.</itunes:summary><pubDate>Thu, 20 Jan 2022 15:39:00 +0000</pubDate><itunes:duration>1865</itunes:duration><enclosure url="http://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/http/vpid/p0bjcm8l.mp3" length="14920000" type="audio/mpeg"/><ppg:enclosureLegacy url="http://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/http/vpid/p0bjcm8l.mp3" length="14920000" type="audio/mpeg"/><ppg:enclosureSecure url="https://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/https/vpid/p0bjcm8l.mp3" length="14920000" type="audio/mpeg"/><guid isPermaLink="false">urn:bbc:podcast:p0bjcrk9</guid><link>http://www.bbc.co.uk/programmes/p0bjcrk9</link><itunes:explicit>clean</itunes:explicit><media:content url="http://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/http/vpid/p0bjcm8l.mp3" fileSize="14920000" type="audio/mpeg" medium="audio" expression="full" duration="1865"/><itunes:author>BBC World Service</itunes:author><ppg:canonical>/programmes/p0bjcrk9</ppg:canonical></item><item><title>Biden believes Putin will move in on Ukraine</title><description>The US President has warned that Washington will do significant harm to Russia if it decides to invade Ukraine. Also: DR Congo inmates found guilty of rape during prison riot, and airlines cancel dozens of flights to the US because of safety concerns surrounding aircraft altimeters and 5G technology.</description><itunes:subtitle>Moscow denies planning to attack or invade</itunes:subtitle><itunes:summary>The US President has warned that Washington will do significant harm to Russia if it decides to invade Ukraine. Also: DR Congo inmates found guilty of rape during prison riot, and airlines cancel dozens of flights to the US because of safety concerns surrounding aircraft altimeters and 5G technology.</itunes:summary><pubDate>Thu, 20 Jan 2022 01:26:00 +0000</pubDate><itunes:duration>1740</itunes:duration><enclosure url="http://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/http/vpid/p0bj7mc5.mp3" length="13920000" type="audio/mpeg"/><ppg:enclosureLegacy url="http://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/http/vpid/p0bj7mc5.mp3" length="13920000" type="audio/mpeg"/><ppg:enclosureSecure url="https://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/https/vpid/p0bj7mc5.mp3" length="13920000" type="audio/mpeg"/><guid isPermaLink="false">urn:bbc:podcast:p0bj7mzd</guid><link>http://www.bbc.co.uk/programmes/p0bj7mzd</link><itunes:explicit>clean</itunes:explicit><media:content url="http://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/http/vpid/p0bj7mc5.mp3" fileSize="13920000" type="audio/mpeg" medium="audio" expression="full" duration="1740"/><itunes:author>BBC World Service</itunes:author><ppg:canonical>/programmes/p0bj7mzd</ppg:canonical></item><item><title>British PM under increasing pressure to go</title><description>Boris Johnson is accused of misleading people about lockdown parties; US urges Russia to take "peaceful path" as fears mount of a new invasion of Ukraine. And we remember the ground breaking life of the African American journalist André Leon Talley.</description><itunes:subtitle>Boris Johnson is accused of misleading people about lockdown parties</itunes:subtitle><itunes:summary>Boris Johnson is accused of misleading people about lockdown parties; US urges Russia to take "peaceful path" as fears mount of a new invasion of Ukraine. And we remember the ground breaking life of the African American journalist André Leon Talley.</itunes:summary><pubDate>Wed, 19 Jan 2022 15:24:00 +0000</pubDate><itunes:duration>1678</itunes:duration><enclosure url="http://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/http/vpid/p0bj2nr7.mp3" length="13424000" type="audio/mpeg"/><ppg:enclosureLegacy url="http://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/http/vpid/p0bj2nr7.mp3" length="13424000" type="audio/mpeg"/><ppg:enclosureSecure url="https://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/https/vpid/p0bj2nr7.mp3" length="13424000" type="audio/mpeg"/><guid isPermaLink="false">urn:bbc:podcast:p0bj2srp</guid><link>http://www.bbc.co.uk/programmes/p0bj2srp</link><itunes:explicit>clean</itunes:explicit><media:content url="http://open.live.bbc.co.uk/mediaselector/6/redir/version/2.0/mediaset/audio-nondrm-download-low/proto/http/vpid/p0bj2nr7.mp3" fileSize="13424000" type="audio/mpeg" medium="audio" expression="full" duration="1678"/><itunes:author>BBC World Service</itunes:author><ppg:canonical>/programmes/p0bj2srp</ppg:canonical></item></channel></rss>"#);
		});

		let res: Vec<Episode> = retreive_episodes(server.base_url(), 1).unwrap();

		assert!(true);
		assert_eq!(res[0].title, String::from("Former Pope Benedict accused of failing to act over child abuse cases"));

	}
}