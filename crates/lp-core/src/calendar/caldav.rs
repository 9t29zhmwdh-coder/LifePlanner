use crate::models::*;
use chrono::{DateTime, Utc};
use reqwest::Client;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CalDavError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Auth error: invalid credentials")]
    Auth,
    #[error("Parse error: {0}")]
    Parse(String),
}

pub async fn sync_caldav(
    account: &CalendarAccount,
    password: &str,
) -> Result<Vec<Event>, CalDavError> {
    let url = account.url.as_deref().unwrap_or("");
    let username = account.username.as_deref().unwrap_or("");

    let client = Client::builder()
        .danger_accept_invalid_certs(false)
        .build()?;

    // REPORT query to fetch all VEVENT components in the last 90 + next 365 days
    let now = Utc::now();
    let from = format_caldav_date(now - chrono::Duration::days(90));
    let to   = format_caldav_date(now + chrono::Duration::days(365));

    let body = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<c:calendar-query xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav">
  <d:prop><d:getetag/><c:calendar-data/></d:prop>
  <c:filter>
    <c:comp-filter name="VCALENDAR">
      <c:comp-filter name="VEVENT">
        <c:time-range start="{}" end="{}"/>
      </c:comp-filter>
    </c:comp-filter>
  </c:filter>
</c:calendar-query>"#, from, to);

    let resp = client
        .request(reqwest::Method::from_bytes(b"REPORT").unwrap(), url)
        .basic_auth(username, Some(password))
        .header("Depth", "1")
        .header("Content-Type", "application/xml")
        .body(body)
        .send()
        .await?;

    if resp.status() == 401 {
        return Err(CalDavError::Auth);
    }

    let text = resp.text().await?;
    let events = parse_caldav_response(&text, &account.id);
    Ok(events)
}

fn format_caldav_date(dt: DateTime<Utc>) -> String {
    dt.format("%Y%m%dT%H%M%SZ").to_string()
}

fn parse_caldav_response(xml: &str, calendar_id: &str) -> Vec<Event> {
    let mut events = Vec::new();

    // Extract calendar-data blocks from the multistatus response
    let data_re = regex::Regex::new(r"<.*?calendar-data[^>]*>([\s\S]*?)</.*?calendar-data>").unwrap();

    for cap in data_re.captures_iter(xml) {
        let ics_data = cap[1].trim();
        // Write to temp string and parse as ICS
        if let Ok(parsed) = parse_ics_string(ics_data, calendar_id) {
            events.extend(parsed);
        }
    }

    events
}

fn parse_ics_string(data: &str, calendar_id: &str) -> Result<Vec<Event>, ()> {
    use ical::IcalParser;
    use std::io::BufReader;
    use chrono::{NaiveDateTime, TimeZone};

    let reader = BufReader::new(data.as_bytes());
    let parser = IcalParser::new(reader);
    let mut result = Vec::new();

    for calendar in parser.flatten() {
        for component in calendar.events {
            let props = &component.properties;
            let get = |name: &str| -> Option<String> {
                props.iter()
                    .find(|p| p.name == name)
                    .and_then(|p| p.value.clone())
            };

            let title = get("SUMMARY").unwrap_or_else(|| "Termin".into());
            let start_str = get("DTSTART").unwrap_or_default();
            let start = parse_dt(&start_str).unwrap_or_else(Utc::now);
            let end   = get("DTEND").and_then(|s| parse_dt(&s));

            let mut ev = Event::new(title, start);
            ev.source = EventSource::CalDav;
            ev.calendar_id = Some(calendar_id.to_string());
            ev.external_uid = get("UID");
            ev.description = get("DESCRIPTION");
            ev.location = get("LOCATION");
            ev.end = end;
            ev.all_day = start_str.len() == 8;
            result.push(ev);
        }
    }

    Ok(result)
}

fn parse_dt(s: &str) -> Option<chrono::DateTime<Utc>> {
    let s = s.trim_end_matches('Z').replace('T', "");
    let fmt = if s.len() >= 14 { "%Y%m%d%H%M%S" } else { "%Y%m%d000000" };
    let s = if s.len() == 8 { format!("{}000000", s) } else { s[..14.min(s.len())].to_string() };
    chrono::NaiveDateTime::parse_from_str(&s, fmt)
        .ok()
        .and_then(|ndt| chrono::Utc.from_local_datetime(&ndt).single())
}
