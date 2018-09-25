
extern crate reqwest;
extern crate json;

pub mod search_result;
use super::get_api_key;
use reqwest::StatusCode;

pub fn search(query: String, max_results: String) -> Option<Vec<search_result::Video>> {
    let mut response = reqwest::get(&format!("https://www.googleapis.com/youtube/v3/search?q={}&maxResults={}&part=snippet&key={}", &query, &max_results, &get_api_key())).expect("An error occurred when fetching the youtube API.");
    match response.status() {
        StatusCode::OK => {
            match response.text() {
                Ok(text) => {
                    let data = json::parse(text.as_str()).unwrap();
                    let mut index = 0;
                    let mut results: Vec<search_result::Video> = data["items"].members().map(|video_object| {
                        index += 1;
                        let title = &video_object["snippet"]["title"];
                        let description = &video_object["snippet"]["description"];
                        let published_at = &video_object["snippet"]["publishedAt"];
                        let channel_id = &video_object["snippet"]["channelId"];
                        let channel_title = &video_object["snippet"]["channelTitle"];
                        let video_id = &video_object["id"]["videoId"];
                        search_result::Video::from_object(index, video_id.to_string(), published_at.to_string(), channel_id.to_string(), title.to_string(), description.to_string(), channel_title.to_string())
                    }).collect();
                    Some(results)
                },
                Err(_) => panic!("Unable to get the content of the body.")
            }
        },
        StatusCode::NOT_FOUND => None,
        s => panic!("An error occurred when calling the youtube API. Status: {}", s)
    }
}