use egg_mode;
use std::env;

pub fn auth_app() -> egg_mode::Token {
    egg_mode::Token::Access {
        consumer: egg_mode::KeyPair::new(
            env::var("TWITTER_CONSUMER").expect("need 'TWITTER_CONSUMER' in env var"),
            env::var("TWITTER_CONSUMER_SECRET").expect("need 'TWITTER_CONSUMER_SECRET' in env var"),
        ),
        access: egg_mode::KeyPair::new(
            env::var("TWITTER_ACCESS").expect("need 'TWITTER_ACCESS' in env var"),
            env::var("TWITTER_ACCESS_SECRET").expect("need 'TWITTER_ACCESS_SECRET' in env var"),
        ),
    }
}

pub fn make_query(raw_query: &str) -> String {
    let mut joined = raw_query
        .split('+')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|e| format!("\"{}\"", e))
        .collect::<Vec<_>>()
        .join(" OR ");
    joined.push_str(" exclude:retweets");
    joined.push_str(" filter:images");
    joined
} 