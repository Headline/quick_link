#[macro_use] extern crate rocket;

use std::sync::{Arc};
use lru_cache::LruCache;
use rocket::response::Redirect;
use rocket::State;
use rand::distributions::{Alphanumeric, DistString};
use rocket::tokio::sync::Mutex;

const DEFAULT_CACHE_SIZE : usize = 10000;

struct LinkStore {
    store : Arc<Mutex<LruCache<String, String>>>
}

#[get("/<code>")]
async fn index(code : String, link_store : &State<LinkStore>) -> Redirect {
    let mut link_reader = link_store.store.lock().await;
    let maybe_url = link_reader.get_mut(&code);
    if let Some(url) = maybe_url {
        println!("Redirecting to: '{}'", &url);
        Redirect::to(url.clone())
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
    let rocket = rocket::build();

    // Comes from Rocket.toml or ROCKET_CACHE_SIZE, like the rest of the config.
    let cache_size = rocket.figment()
        .extract_inner("cache_size")
        .unwrap_or(DEFAULT_CACHE_SIZE);
    println!("Cache size: {}", cache_size);

    let _rocket = rocket
        .mount("/", routes![index, link])
        .manage(
        LinkStore {
            store: Arc::new(Mutex::new(LruCache::new(cache_size)))
        }
    ).launch().await?;

    Ok(())
}