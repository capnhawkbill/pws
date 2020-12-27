use rocket_contrib::databases::rusqlite;
use anyhow::Result;
use csv::{Reader, Writer};

pub mod models;

#[database("sqlite_database")]
pub struct DbConn(rusqlite::Connection);

/// Make a csv string
fn mkcsv(thing: &[String]) -> Result<String> {
    let mut wtr = Writer::from_writer(vec![]);
    wtr.write_record(thing)?;
    Ok(String::from_utf8(wtr.into_inner()?)?)
}

/// Parse a csv string
fn getcsv(thing: String) -> Result<Vec<String>> {
    let mut rdr = Reader::from_reader(thing.as_bytes());
    let mut r = Vec::new();
    for record in rdr.records() {
        let record = record?;
        r.push(record.as_slice().to_string())
    }
    Ok(r)
}
