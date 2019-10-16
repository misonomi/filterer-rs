#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use rocket::{routes};

mod twitter;
mod router;


fn main() {
    let twitter_token = twitter::auth_app();
    rocket::ignite()
        .manage(twitter_token)
        .mount("/", routes![router::hello, router::search])
        .launch();
}
