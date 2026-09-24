use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc, Weekday};
use once_cell::sync::Lazy;
use regex::Regex;

static RE_ISO: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{4})-(\d{2})-(\d{2})(?:[T ](\d{2}):(\d{2})(?::(\d{2}))?)?\b").unwrap()
});
static RE_DE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{1,2})\.\s*(\d{1,2})\.(?:\s*(\d{4}))?\b").unwrap()
});
static RE_TIME_24: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([01]?\d|2[0-3]):([0-5]\d)\b").unwrap()
});
static RE_RELATIVE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b(heute|morgen|übermorgen|today|tomorrow|nächste(?:n|r|s)?|next)\s*(woche|week|montag|dienstag|mittwoch|donnerstag|freitag|samstag|sonntag|monday|tuesday|wednesday|thursday|friday|saturday|sunday)?\b").unwrap()
});
static RE_IN_N: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\bin\s+(\d+)\s+(tag(?:en)?|woche(?:n)?|monat(?:en)?|day(?:s)?|week(?:s)?|month(?:s)?)\b").unwrap()
});
static RE_DEADLINE: Lazy<Regex> = Lazy::new(|| {
    // The date may contain dots ("bis 10.10.2026"); a stop at the first dot lost it.
    Regex::new(r"(?i)\b(?:bis|deadline|frist|fällig|due|by|until)\s*:?\s*([^\n,]{3,30})").unwrap()
});

#[derive(Debug, Clone)]
pub struct ExtractedDate {
    pub datetime: DateTime<Utc>,
    pub is_deadline: bool,
    pub raw: String,
}

/// "um 14:30" in a letter means 14:30 where the reader lives; reading it as UTC
/// moved every appointment by the local offset (two hours in Swiss summer).
pub fn local_to_utc(naive: NaiveDateTime) -> Option<DateTime<Utc>> {
    Local.from_local_datetime(&naive).earliest().map(|d| d.with_timezone(&Utc))
}

pub fn extract_dates(text: &str) -> Vec<ExtractedDate> {
    let now = Local::now();
    let mut results: Vec<ExtractedDate> = Vec::new();

    // ISO dates
    for cap in RE_ISO.captures_iter(text) {
        let y: i32 = cap[1].parse().unwrap_or(0);
        let m: u32 = cap[2].parse().unwrap_or(0);
        let d: u32 = cap[3].parse().unwrap_or(0);
        let h: u32 = cap.get(4).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
        let min: u32 = cap.get(5).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
        if let Some(dt) = NaiveDate::from_ymd_opt(y, m, d)
            .and_then(|date| date.and_hms_opt(h, min, 0))
            .and_then(local_to_utc)
        {
            results.push(ExtractedDate { datetime: dt, is_deadline: false, raw: cap[0].to_string() });
        }
    }

    // German dates: 15.3. or 15.3.2025
    for cap in RE_DE.captures_iter(text) {
        let d: u32 = cap[1].parse().unwrap_or(0);
        let m: u32 = cap[2].parse().unwrap_or(0);
        let y: i32 = cap.get(3).and_then(|c| c.as_str().parse().ok())
            .unwrap_or(now.year());
        if let Some(date) = NaiveDate::from_ymd_opt(y, m, d) {
            let h = extract_nearby_time(text, cap.get(0).unwrap().start());
            let Some(dt) = local_to_utc(date.and_time(h)) else { continue };
            results.push(ExtractedDate { datetime: dt, is_deadline: false, raw: cap[0].to_string() });
        }
    }

    // Relative dates
    for cap in RE_RELATIVE.captures_iter(text) {
        let anchor = cap[1].to_lowercase();
        let dt = match anchor.as_str() {
            "heute" | "today" => Some(now.date_naive()),
            "morgen" | "tomorrow" => Some((now + Duration::days(1)).date_naive()),
            "übermorgen" => Some((now + Duration::days(2)).date_naive()),
            _ => {
                // "nächsten Montag" etc.
                cap.get(2).and_then(|w| weekday_offset(w.as_str(), &now))
            }
        };
        if let Some(date) = dt {
            let h = extract_nearby_time(text, cap.get(0).unwrap().start());
            let Some(dt) = local_to_utc(date.and_time(h)) else { continue };
            results.push(ExtractedDate { datetime: dt, is_deadline: false, raw: cap[0].to_string() });
        }
    }

    // "in N days/weeks/months"
    for cap in RE_IN_N.captures_iter(text) {
        let n: i64 = cap[1].parse().unwrap_or(1);
        let unit = cap[2].to_lowercase();
        let dt = if unit.starts_with("tag") || unit.starts_with("day") {
            now + Duration::days(n)
        } else if unit.starts_with("woch") || unit.starts_with("week") {
            now + Duration::weeks(n)
        } else {
            now + Duration::days(n * 30)
        };
        results.push(ExtractedDate { datetime: dt.with_timezone(&Utc), is_deadline: false, raw: cap[0].to_string() });
    }

    // Deadline phrases: mark as deadline
    // The same date was already found as a plain date; mark that one instead of
    // adding a second copy, which could then be taken for an appointment.
    for cap in RE_DEADLINE.captures_iter(text) {
        for found in extract_dates(&cap[1]) {
            match results.iter_mut().find(|r| (r.datetime - found.datetime).num_minutes().abs() < 5) {
                Some(existing) => existing.is_deadline = true,
                None => results.push(ExtractedDate { is_deadline: true, ..found }),
            }
        }
    }

    results.dedup_by(|a, b| (a.datetime - b.datetime).num_minutes().abs() < 5);
    results
}

fn extract_nearby_time(text: &str, pos: usize) -> NaiveTime {
    // Byte offsets can land inside an umlaut; move to the nearest character boundary.
    let mut from = pos.saturating_sub(50);
    while !text.is_char_boundary(from) { from -= 1; }
    let mut to = (pos + 80).min(text.len());
    while !text.is_char_boundary(to) { to += 1; }
    let window = &text[from..to];
    RE_TIME_24.captures(window)
        .and_then(|c| {
            let h: u32 = c[1].parse().ok()?;
            let m: u32 = c[2].parse().ok()?;
            NaiveTime::from_hms_opt(h, m, 0)
        })
        .unwrap_or_else(|| NaiveTime::from_hms_opt(9, 0, 0).unwrap())
}

fn weekday_offset(name: &str, now: &DateTime<Local>) -> Option<NaiveDate> {
    let target = match name.to_lowercase().as_str() {
        "montag" | "monday"    => Weekday::Mon,
        "dienstag" | "tuesday" => Weekday::Tue,
        "mittwoch" | "wednesday" => Weekday::Wed,
        "donnerstag" | "thursday" => Weekday::Thu,
        "freitag" | "friday"   => Weekday::Fri,
        "samstag" | "saturday" => Weekday::Sat,
        "sonntag" | "sunday"   => Weekday::Sun,
        _ => return None,
    };
    let today = now.weekday();
    let today_n = today.num_days_from_monday();
    let target_n = target.num_days_from_monday();
    let diff = if target_n > today_n { target_n - today_n } else { 7 - today_n + target_n };
    Some(now.date_naive() + Duration::days(diff as i64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn a_time_in_the_text_is_local_time() {
        let found = extract_dates("Termin am 12.10.2026 um 14:30");
        let local = found[0].datetime.with_timezone(&Local);
        assert_eq!((local.hour(), local.minute()), (14, 30));
    }

    #[test]
    fn umlauts_around_a_date_do_not_panic() {
        let text = format!("{} 12.10.2026 um 9:15 {}", "ä".repeat(40), "ü".repeat(60));
        let local = extract_dates(&text)[0].datetime.with_timezone(&Local);
        assert_eq!((local.hour(), local.minute()), (9, 15));
    }
}
