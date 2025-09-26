use std::net::SocketAddr;
use std::str::FromStr;

use warp::{Filter, serve};

#[tokio::main]
async fn main() {
    let hi = warp::path("code")
        .and(warp::path::param())
        //.and(warp::header("???"))
        .map(|param: String| format!("Param: {param}"));
    let server = serve(hi);
    server
        .run(SocketAddr::from_str("127.0.0.1:5000").expect("no valid socket-addr"))
        .await;
    println!("Hello, world!");
}
