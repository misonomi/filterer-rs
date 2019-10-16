use rocket::{get, State, http::RawStr};
use egg_mode::search::{self, ResultType};
pub use yansi::Paint;
use chrono;

use tokio::runtime::current_thread::block_on_all;
#[get("/")]
pub fn hello() -> &'static str {
    "Hello, Rust 2018!"
}

#[get("/search/<terms>")]
pub fn search(tw_token: State<egg_mode::Token>, terms: &RawStr) -> &'static str {
    let search = block_on_all(
        search::search(terms.as_str())
            .result_type(ResultType::Recent)
            .count(10)
            .call(&tw_token),
    )
    .unwrap();

    for tweet in &search.statuses {
        print_tweet(tweet);
    }
    "aaa"
}

fn print_tweet(tweet: &egg_mode::tweet::Tweet) {
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