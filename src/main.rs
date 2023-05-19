use chrono::{DateTime, Utc};
use duckdb::arrow::record_batch::RecordBatch;
use duckdb::arrow::util::pretty::print_batches;
use duckdb::{params, Connection, Result};
use uuid::Uuid;

#[derive(Debug)]
struct Event {
    id: Uuid,
    r#type: String,
    level: String,
    timestamp: DateTime<Utc>,
    source: String,
    payload: String,
    trigger: String,
    category: String,
    account_id: Uuid,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

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
    conn.execute(
        "INSERT INTO event (id, type, level, timestamp, source, payload, trigger, category, account_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![event.id, event.r#type, event.level, event.timestamp, event.source, event.payload, event.trigger, event.category, event.account_id],
    )?;

    // query table by rows
    let mut stmt = conn.prepare("SELECT id, account_id FROM event")?;
    let events = stmt.query_map([], |row| {
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

    for event in events {
        println!("Found event {:?}", event.unwrap());
    }

    // query table by arrow
    let rbs: Vec<RecordBatch> = stmt.query_arrow([])?.collect();
    print_batches(&rbs).unwrap();
    Ok(())
}
