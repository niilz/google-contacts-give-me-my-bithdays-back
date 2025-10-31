use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;

use backend::sync::sync_contact_birthdays;
use clap::Parser;
use warp::reply::Response;
use warp::{Filter, serve};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 's', long)]
    client_secret: String,
    #[arg(short = 'i', long)]
    client_id: String,
}

#[tokio::main]
async fn main() {
    // Do it here to check our program is started correctly
    let _ = Args::parse();

    // Redirect-receiver for access_token
    let code_filter = warp::path("code")
        .and(warp::query::<HashMap<String, String>>())
        .map(|params: HashMap<String, String>| {
            let Some(code) = params.get("code") else {
                eprintln!("oops, no code?");
                panic!("TODO: Responde with: oops no code!");
            };
            println!("got code: {code}");
            // call again here so the path closure does not have to capture the environment
            let args = Args::parse();
            (code.to_string(), args.client_id, args.client_secret)
        })
        .then(
            async move |(code, client_id, client_secret): (String, String, String)| {
                match sync_contact_birthdays(&code, &client_id, &client_secret).await {
                    Ok(()) => Response::new("All went well ;)".into()),
                    Err(e) => Response::new(format!("ERROR: {e}").into()),
                }
            },
        );

    let server = serve(code_filter);
    server
        .run(SocketAddr::from_str("127.0.0.1:5000").expect("no valid socket-addr"))
        .await;
}
