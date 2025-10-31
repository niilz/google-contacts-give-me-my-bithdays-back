use std::collections::BTreeSet;

use crate::birthday::Birthday;
pub use crate::http::{fetch_birthdays, get_or_create_calendar, request_tokens};

pub async fn sync_contact_birthdays(
    code: &str,
    client_id: &str,
    client_secret: &str,
) -> anyhow::Result<()> {
    let access_token = request_tokens(&code, &client_id, &client_secret).await?;
    //println!("Auth-Tokens: {oauth_tokens:?}");
    let birthdays = fetch_birthdays(&access_token).await?;
    //println!("BDays: {birthdays:?}");
    render_birthdays(&birthdays);
    let calendar_id = get_or_create_calendar(&access_token).await?;
    // TODO:
    // Insert Birthday-Entries for every contact with Birthday
    // Ensure no double inserts (probably first clear everything, or check if all ids from existing
    // calendar are identical to the new ones, if something like a unique-ID exists and is stored
    // with eh entry)
    Ok(())
}

fn render_birthdays(birthdays: &BTreeSet<Birthday>) {
    for birthday in birthdays {
        println!("{birthday}")
    }
}
