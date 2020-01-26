use rocket::{get, State, http::RawStr};
use rocket_contrib::json::Json;
use egg_mode::search::{self, ResultType};
use percent_encoding::{percent_decode_str};

use tokio::runtime::current_thread::block_on_all;
use super::twitter::{make_query, shoud_display, print_tweet};
use super::structs::TweetStub;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, Rust 2018!"
}

#[get("/search/<terms>")]
pub fn search(tw_token: State<egg_mode::Token>, terms: &RawStr) -> Json<Vec<TweetStub>> {
    let terms_decoded: String;
    match percent_decode_str(terms).decode_utf8() {
        Ok(v) => terms_decoded = v.to_string(),
        Err(_) => return Json(vec![]),
    }
    let terms_vec = terms_decoded
        .split('+')
        .collect::<Vec<&str>>();
    let search = block_on_all(
        search::search(make_query(terms_vec.clone()))
            .result_type(ResultType::Recent)
            .count(100)
            .call(&tw_token)
    ).unwrap();

    for tweet in &search.statuses {
        print_tweet(tweet);
    }
    Json(
        search.statuses.clone()
            .into_iter()
            .filter(|t| shoud_display(t, &terms_vec))
            .map(|t| TweetStub::from(t))
            .collect()
    )
}
