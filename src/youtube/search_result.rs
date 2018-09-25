extern crate rafy;
use rafy::Rafy;

#[derive(Debug, Clone)]
pub struct Video {
    pub id: i32,
    pub videoId: String,
    pub publishedAt: String,
    pub channelId: String,
    pub title: String,
    pub description: String,
    pub channelTitle: String
}

impl Video {
    pub fn from_object(id: i32, videoId: String, publishedAt: String, channelId: String, title: String, description: String, channelTitle: String) -> Video {
        Video{
            id,
            videoId,
            publishedAt,
            channelId,
            title,
            description,
            channelTitle
        }
    }

    pub fn download(&self) -> Result<(), rafy::Error> {
        let content = Rafy::new(&format!("https://www.youtube.com/watch?v={}", self.videoId))?;
        println!("{}", &format!("https://youtube.com/watch?v={}", self.videoId));
        let stream = &content.streams[0];
        match stream.download(&content.title) {
            Ok(_) => Ok(()),
            Err(_) => Err(rafy::Error::VideoNotFound)
        }
    }

} 