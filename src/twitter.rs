use egg_mode::{Token, KeyPair, tweet::Tweet};
use std::env;
pub use yansi::Paint;
use chrono;

pub fn auth_app() -> Token {
    Token::Access {
        consumer: KeyPair::new(
            env::var("TWITTER_CONSUMER").expect("need 'TWITTER_CONSUMER' in env var"),
            env::var("TWITTER_CONSUMER_SECRET").expect("need 'TWITTER_CONSUMER_SECRET' in env var"),
        ),
        access: KeyPair::new(
            env::var("TWITTER_ACCESS").expect("need 'TWITTER_ACCESS' in env var"),
            env::var("TWITTER_ACCESS_SECRET").expect("need 'TWITTER_ACCESS_SECRET' in env var"),
        ),
    }
}

pub fn make_query(query_vec: Vec<&str>) -> String {
    let mut joined = query_vec.iter()
        .map(|e| format!("\"{}\"", e))
        .collect::<Vec<_>>()
        .join(" OR ");
    joined.push_str(" exclude:retweets");
    joined.push_str(" filter:images");
    joined
}

pub fn shoud_display(tweet: &Tweet, query_vec: &Vec<&str>) -> bool {
    query_vec.iter().any(|q| tweet.text.contains(q))
}

pub fn print_tweet(tweet: &Tweet) {
    if let Some(ref user) = tweet.user {
        println!(
            "{} (@{}) posted at {}",
            Paint::blue(&user.name),
            Paint::bold(Paint::blue(&user.screen_name)),
            tweet.created_at.with_timezone(&chrono::Local)
        );
    }

    if let Some(ref screen_name) = tweet.in_reply_to_screen_name {
        println!("➜ in reply to @{}", Paint::blue(screen_name));
    }

    if let Some(ref status) = tweet.retweeted_status {
        println!("{}", Paint::red("Retweet ➜"));
        print_tweet(status);
        return;
    } else {
        println!("{}", Paint::green(&tweet.text));
    }

    println!("➜ via {} ({})", tweet.source.name, tweet.source.url);

    if let Some(ref place) = tweet.place {
        println!("➜ from: {}", place.full_name);
    }

    if let Some(ref status) = tweet.quoted_status {
        println!("{}", Paint::red("➜ Quoting the following status:"));
        print_tweet(status);
    }

    if !tweet.entities.hashtags.is_empty() {
        println!("➜ Hashtags contained in the tweet:");
        for tag in &tweet.entities.hashtags {
            println!("  {}", tag.text);
        }
    }

    if !tweet.entities.symbols.is_empty() {
        println!("➜ Symbols contained in the tweet:");
        for tag in &tweet.entities.symbols {
            println!("  {}", tag.text);
        }
    }

    if !tweet.entities.urls.is_empty() {
        println!("➜ URLs contained in the tweet:");
        for url in &tweet.entities.urls {
            if let Some(expanded_url) = &url.expanded_url {
                println!("  {}", expanded_url);
            }
        }
    }

    if !tweet.entities.user_mentions.is_empty() {
        println!("➜ Users mentioned in the tweet:");
        for user in &tweet.entities.user_mentions {
            println!("  {}", Paint::bold(Paint::blue(&user.screen_name)));
        }
    }

    if let Some(ref media) = tweet.extended_entities {
        println!("➜ Media attached to the tweet:");
        for info in &media.media {
            println!("  A {:?}", info.media_type);
        }
    }
}