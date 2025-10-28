use std::collections::BTreeSet;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;

use backend::Birthday;
use backend::api::{AuthTokens, Connections};
use clap::Parser;
use reqwest::header::HeaderMap;
use warp::reply::Response;
use warp::{Filter, serve};

const REDIRECT_URI_DEV: &str = "http://localhost:5000/code";
const PEOPLE_API_BASE_URL: &str = "https://people.googleapis.com/v1";

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
                let birthdays = fetch_birthdays(&code, &client_id, &client_secret).await;
                println!("BDays: {birthdays:?}");
                Response::new(format!("Thank you for the code: {code}").into())
            },
        );

    let server = serve(code_filter);
    server
        .run(SocketAddr::from_str("127.0.0.1:5000").expect("no valid socket-addr"))
        .await;
    println!("Hello, world!");
}

async fn fetch_birthdays(code: &str, client_id: &str, client_secret: &str) -> anyhow::Result<()> {
    let oauth_tokens = request_tokens(code, client_id, client_secret).await?;
    println!("Auth-Tokens: {oauth_tokens:?}");
    let client = reqwest::Client::new();

    // Use max-page size so we need as little requests as possible
    let connections_url = format!(
        "{PEOPLE_API_BASE_URL}/people/me/connections?personFields=names,birthdays&pageSize=1000"
    );

    // First page (no next-page-token)
    let mut connections =
        load_connections(&client, &connections_url, &oauth_tokens.access_token).await?;

    let mut next_page_token = connections.next_page_token.clone();
    loop {
        if next_page_token.is_none() {
            break;
        }
        let next_connections = load_connections(
            &client,
            &format!("{connections_url}&pageToken={}", next_page_token.unwrap()),
            &oauth_tokens.access_token,
        )
        .await?;
        next_page_token = next_connections.next_page_token.clone();
        connections.connections.extend(next_connections.connections);
    }

    let birthdays: BTreeSet<Birthday> = connections
        .connections
        .into_iter()
        .filter_map(|person| person.try_into().ok())
        .collect();

    render_birthdays(&birthdays);
    println!("Total: {} birthday entries", birthdays.len());

    // write birthdays to calendar
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", &oauth_tokens.access_token)
            .parse()
            .unwrap(),
    );
    let calendars = client
        .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    println!("{calendars}");
    Ok(())
}

async fn load_connections(
    client: &reqwest::Client,
    connections_url: &str,
    access_token: &str,
) -> anyhow::Result<Connections> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", access_token).parse().unwrap(),
    );

    let connections = client.get(connections_url).headers(headers).send().await?;
    //println!("{connections:?}");
    let connections = connections.json::<Connections>().await?;

    Ok(connections)
}

fn render_birthdays(birthdays: &BTreeSet<Birthday>) {
    for birthday in birthdays {
        println!("{birthday}")
    }
}

async fn request_tokens(
    code: &str,
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<AuthTokens> {
    let token_request_url = "https://oauth2.googleapis.com/token";
    let mut oauth_data = HashMap::new();
    oauth_data.insert("code", code);
    oauth_data.insert("client_id", client_id);
    oauth_data.insert("client_secret", client_secret);
    oauth_data.insert("redirect_uri", REDIRECT_URI_DEV);
    oauth_data.insert("grant_type", "authorization_code");
    let client = reqwest::Client::new();
    let oauth_tokens: AuthTokens = client
        .post(token_request_url)
        .form(&oauth_data)
        .send()
        .await?
        .json()
        .await?;
    Ok(oauth_tokens)
}
