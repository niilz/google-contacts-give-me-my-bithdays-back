use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;

use warp::reply::Response;
use warp::{Filter, serve};

#[tokio::main]
async fn main() {
    let hi = warp::path("code")
        .and(warp::query::<HashMap<String, String>>())
        .map(|map| {
            for (key, value) in map {
                println!("key: {key}, value: {value}");
            }
            Response::new("foo".into())
        });
    let server = serve(hi);
    server
        .run(SocketAddr::from_str("127.0.0.1:5000").expect("no valid socket-addr"))
        .await;
    println!("Hello, world!");
}
