extern crate colored;
extern crate reqwest;
extern crate rafy;
use std::env;
pub mod youtube;


pub fn is_logged() -> bool {
    match env::var("YOUTUBE_API_KEY") {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn get_api_key() -> String {
    env::var("YOUTUBE_API_KEY").unwrap()
}