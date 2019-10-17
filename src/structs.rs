use serde::{Serialize};
use egg_mode::tweet::Tweet;

#[derive(Serialize)]
pub struct TweetStub {
    text: String,
    images: Vec<Image>,
}

impl From<Tweet> for TweetStub {
    fn from(tweet: Tweet) -> Self {
        let image: Vec<Image>;
        if let Some(media) = tweet.extended_entities {
            image = media.media
                .into_iter()
                .map(|m| {
                    Image{
                        url: m.media_url_https,
                        W: 0,
                        H: 0,
                    }
                })
                .collect::<Vec<_>>();
        } else {
            image = Vec::new();
        }
        TweetStub{
            text: tweet.text,
            images: image,
        }
    }
}

#[derive(Serialize)]
struct Image {
    url: String,
    W: u16,
    H: u16,
}