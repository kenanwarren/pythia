use anyhow::Error;
use duckdb::{params, Connection};
use pythia_core::Event;

pub fn migrate(conn: &Connection) -> Result<(), Error> {
    conn.execute_batch(
        r"
        CREATE TABLE event (
            id              UUID PRIMARY KEY DEFAULT NEXTVAL(uuid()),
            type            TEXT NOT NULL,
            level           TEXT NOT NULL,
            timestamp       TIMESTAMPTZ NOT NULL,
            source          TEXT NOT NULL,
            payload         TEXT NOT NULL,
            trigger         TEXT NOT NULL,
            category        TEXT NOT NULL,
            account_id      UUID NOT NULL
        );
        ",
    )?;

    Ok(())
}

pub fn create_event(conn: &Connection, event: Event) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO event (id, type, level, timestamp, source, payload, trigger, category, account_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![event.id, event.r#type, event.level, event.timestamp, event.source, event.payload, event.trigger, event.category, event.account_id],
    )?;

    Ok(())
}

pub fn get_events(conn: &Connection) -> Result<Vec<Event>, Error> {
    let mut events = Vec::new();
    let mut stmt = conn.prepare("SELECT id, account_id FROM event")?;
    let results = stmt.query_map([], |row| {
        Ok(Event {
            id: row.get(0)?,
            r#type: String::from(""),
            level: String::from(""),
            timestamp: chrono::offset::Utc::now(),
            source: String::from(""),
            payload: String::from(""),
            trigger: String::from(""),
            category: String::from(""),
            account_id: row.get(1)?,
        })
    })?;

    for result in results {
        events.push(result.unwrap());
    }

    Ok(events)
}
