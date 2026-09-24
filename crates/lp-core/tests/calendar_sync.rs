use chrono::{Duration, Utc};
use lp_core::db::{get_all_events, replace_calendar_events, Database};
use lp_core::models::Event;

fn event(uid: &str, title: &str, hours_from_now: i64) -> Event {
    let start = Utc::now() + Duration::hours(hours_from_now);
    let mut e = Event::new(title, start);
    e.end = Some(start + Duration::hours(1));
    e.external_uid = Some(uid.to_string());
    e
}

async fn database() -> (tempfile::TempDir, Database) {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("test.db")).await.unwrap();
    (dir, db)
}

#[tokio::test]
async fn syncing_twice_does_not_double_the_calendar() {
    let (_dir, db) = database().await;
    let feed = || vec![event("a@cal", "Dentist", 2), event("b@cal", "Standup", 5)];
    replace_calendar_events(&db, "work", feed()).await.unwrap();
    replace_calendar_events(&db, "work", feed()).await.unwrap();
    assert_eq!(get_all_events(&db).await.unwrap().len(), 2);
}

#[tokio::test]
async fn changed_events_are_updated_in_place() {
    let (_dir, db) = database().await;
    replace_calendar_events(&db, "work", vec![event("a@cal", "Dentist", 2)]).await.unwrap();
    let before = get_all_events(&db).await.unwrap()[0].id.clone();
    replace_calendar_events(&db, "work", vec![event("a@cal", "Dentist (moved)", 3)]).await.unwrap();
    let after = get_all_events(&db).await.unwrap();
    assert_eq!(after.len(), 1);
    assert_eq!(after[0].id, before);
    assert_eq!(after[0].title, "Dentist (moved)");
}

#[tokio::test]
async fn events_deleted_in_the_calendar_disappear() {
    let (_dir, db) = database().await;
    replace_calendar_events(&db, "work", vec![event("a@cal", "Dentist", 2), event("b@cal", "Gone", 4)]).await.unwrap();
    replace_calendar_events(&db, "work", vec![event("a@cal", "Dentist", 2)]).await.unwrap();
    let titles: Vec<_> = get_all_events(&db).await.unwrap().into_iter().map(|e| e.title).collect();
    assert_eq!(titles, vec!["Dentist"]);
}

#[tokio::test]
async fn other_calendars_and_own_events_are_left_alone() {
    let (_dir, db) = database().await;
    replace_calendar_events(&db, "private", vec![event("p@cal", "Dinner", 6)]).await.unwrap();
    let own = Event::new("Written by hand", Utc::now());
    lp_core::db::insert_event(&db, &own).await.unwrap();
    replace_calendar_events(&db, "work", vec![]).await.unwrap();
    assert_eq!(get_all_events(&db).await.unwrap().len(), 2);
}
