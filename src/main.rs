#[macro_use] extern crate rocket;

use std::sync::{Arc};
use lru_cache::LruCache;
use rocket::response::Redirect;
use rocket::State;
use rand::distributions::{Alphanumeric, DistString};
use rocket::tokio::sync::Mutex;

struct LinkStore {
    store : Arc<Mutex<LruCache<String, String>>>
}

#[get("/<code>")]
async fn index(code : String, link_store : &State<LinkStore>) -> Redirect {
    let mut link_reader = link_store.store.lock().await;
    let maybe_url = link_reader.get_mut(&code);
    if let Some(url) = maybe_url {
        let mut trimmed = url.trim().chars();
        trimmed.next_back();
        let url = trimmed.as_str().to_string();
        println!("Redirecting to: '{}'", url);
        Redirect::to(url)
    }
    else {
        Redirect::to("/")
    }
}

#[post("/", data = "<input>", format = "text/plain")]
async fn link(input: String, link_store : &State<LinkStore>) -> String {
    let mut link_writer = link_store.store.lock().await;
    let rand_str = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    link_writer.insert(rand_str.clone(), input);
    rand_str
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, link])
        .manage(
        LinkStore {
            store: Arc::new(Mutex::new(LruCache::new(10000)))
        }
    ).launch().await?;

    Ok(())
}