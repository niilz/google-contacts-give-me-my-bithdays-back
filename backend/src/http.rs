use std::collections::{BTreeSet, HashMap};

use reqwest::header::HeaderMap;

use crate::api::calendar::{Calendar, CalendarList, InsertEvent};
use crate::api::person::{AuthTokens, Connections};
use crate::birthday::Birthday;

const REDIRECT_URI_DEV: &str = "http://localhost:5000/code";
const PEOPLE_API_BASE_URL: &str = "https://people.googleapis.com/v1";
const CALENDAR_API_BASE_URL: &str = "https://www.googleapis.com/calendar/v3";
const DEFAULT_BDAY_CAL: &str = "contacts-birthdays";

pub async fn request_tokens(
    code: &str,
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<String> {
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
    Ok(oauth_tokens.access_token)
}

pub async fn fetch_birthdays(access_token: &str) -> anyhow::Result<BTreeSet<Birthday>> {
    let client = reqwest::Client::new();

    // Use max-page size so we need as little requests as possible
    let connections_url = format!(
        "{PEOPLE_API_BASE_URL}/people/me/connections?personFields=names,birthdays&pageSize=1000"
    );

    // First page (no next-page-token)
    let mut connections = load_connections(&client, &connections_url, access_token).await?;

    let mut next_page_token = connections.next_page_token.clone();
    loop {
        if next_page_token.is_none() {
            break;
        }
        let next_connections = load_connections(
            &client,
            &format!("{connections_url}&pageToken={}", next_page_token.unwrap()),
            access_token,
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

    Ok(birthdays)
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
pub async fn get_or_create_calendar(access_token: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", access_token).parse().unwrap(),
    );
    let calendars: Vec<Calendar> = client
        .get(format!("{CALENDAR_API_BASE_URL}/users/me/calendarList"))
        .headers(headers)
        .send()
        .await?
        .json::<CalendarList>()
        .await?
        .items;

    println!("{calendars:?}");

    let bday_cal = calendars
        .into_iter()
        .find(|c| c.summary == DEFAULT_BDAY_CAL);

    let cal = match bday_cal {
        Some(cal) => cal,
        None => create_cal(access_token).await?,
    };
    //println!("{cal:?}");
    Ok(cal.id)
}

async fn create_cal(access_token: &str) -> anyhow::Result<Calendar> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", access_token).parse().unwrap(),
    );

    println!("creating new calendar");

    let new_cal = client
        .post(format!("{CALENDAR_API_BASE_URL}/calendars"))
        .body(format!("{{'summary': '{DEFAULT_BDAY_CAL}'}}"))
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    Ok(new_cal)
}

pub async fn insert_birthday(
    access_token: &str,
    bday: &Birthday,
    calendar_id: &str,
) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", access_token).parse().unwrap(),
    );
    let insert_event: InsertEvent = bday.into();
    let insert_bday = serde_json::to_string(&insert_event)?;

    //println!("insert_bday: {insert_bday}");

    let new_bday = client
        .post(format!(
            "{CALENDAR_API_BASE_URL}/calendars/{calendar_id}/events"
        ))
        .body(insert_bday)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    println!("{new_bday}");
    //println!("posted: {}", new_bday.summary);

    Ok(())
}
