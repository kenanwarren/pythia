use anyhow::Error;
use duckdb::{Connection, Result};
use pythia_core::Event;
use uuid::Uuid;

mod db;

fn main() -> Result<(), Error> {
    let conn = Connection::open_in_memory()?;

    db::migrate(&conn)?;

    let event = Event {
        id: Uuid::new_v4(),
        r#type: String::from("test"),
        level: String::from("test"),
        timestamp: chrono::offset::Utc::now(),
        source: String::from("test"),
        payload: String::from("test"),
        trigger: String::from("test"),
        category: String::from("test"),
        account_id: Uuid::new_v4(),
    };
    db::create_event(&conn, event)?;

    let events = db::get_events(&conn);
    for event in events {
        println!("Found event {:?}", event);
    }

    Ok(())
}
