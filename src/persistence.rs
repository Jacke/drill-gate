#![feature(trait_alias)]
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;

mod records;
use records::*;

use rusqlite::Error as SQLiteError;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error as IOError, ErrorKind};

use rusqlite::{types::FromSql, types::FromSqlError, types::ValueRef, Row};

//pub trait MyErrorTrait = Error + Clone + Send + Sync;
//trait MyErrorTraitAlias = MyErrorTrait + Send + Sync;
//impl MyErrorTraitAlias for rusqlite::Error {}

// A custom error type that wraps both `IOError` and `SQLiteError`
#[derive(Debug)]
enum CustomError {
    Io(IOError),
    SQLite(SQLiteError),
    UUID(uuid::Error),
    Json(serde_json::Error),
}

// Implement the `Error` trait for the custom error type
impl Error for CustomError {}
impl From<CustomError> for rusqlite::Error {
    fn from(error: CustomError) -> Self {
        println!("{:?}", error);
        rusqlite::Error::QueryReturnedNoRows
    }
}

// Implement the `Display` trait for the custom error type
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Io(e) => write!(f, "I/O error: {}", e),
            CustomError::SQLite(e) => write!(f, "SQLite error: {}", e),
            CustomError::UUID(e) => write!(f, "UUID error: {}", e),
            CustomError::Json(e) => write!(f, "Json error: {}", e),
        }
    }
}

pub fn drop_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    return conn.execute("DROP TABLE IF EXISTS entries", []);
}

pub fn init_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    return conn.execute(
        "CREATE TABLE IF NOT EXISTS entries (
                  id TEXT PRIMARY KEY,
                  entry_type TEXT NOT NULL,
                  version TEXT NOT NULL,
                  meta TEXT NOT NULL,
                  data TEXT NOT NULL
                  )",
        [],
    );
}

pub fn add_entry(conn: &Connection, title: Option<String>) -> Result<(), Box<dyn Error>> {
    println!("Enter the entry type:");
    let mut entry_type = String::new();
    io::stdin().read_line(&mut entry_type)?;

    println!("Enter the entry version:");
    let mut version = String::new();
    io::stdin().read_line(&mut version)?;

    let id = Uuid::new_v4().to_string();

    let meta = Meta {
        created_at: chrono::Utc::now().to_string(),
        updated_at: chrono::Utc::now().to_string(),
        deleted_at: None,
    };

    println!("Enter the entry data (as JSON):");
    let mut data = String::new();
    io::stdin().read_line(&mut data)?;
    let mut data_json: serde_json::Value = serde_json::from_str(&data)?;

    if (title.is_some()) {
        data_json["title"] = serde_json::Value::String(title.unwrap());
    }

    let entry = Entry {
        id: id.clone(),
        entry_type: entry_type.trim().to_owned(),
        version: version.trim().to_owned(),
        meta,
        data: data_json,
    };
    let meta_json = serde_json::to_string(&entry.meta)?;

    conn.execute(
        "INSERT INTO entries (id, entry_type, version, meta, data)
                VALUES (?1, ?2, ?3, ?4, ?5)",
        &[
            &entry.id,
            &entry.entry_type.trim().to_string(),
            &entry.version.trim().to_string(),
            &meta_json,
            &serde_json::to_string(&entry.data).unwrap(),
        ],
    )?;

    println!("Entry added with ID: {}", id);

    Ok(())
}

pub fn remove_entry(conn: &Connection, id: Uuid) -> Result<(), Box<dyn Error>> {
    conn.execute("DELETE FROM entries WHERE id = ?1", params![id.to_string()])?;

    Ok(())
}

pub fn update_entry(
    conn: &Connection,
    id: Uuid,
    data: serde_json::Value,
) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string(&data)?;
    conn.execute(
        "UPDATE entries SET data = ?1 WHERE id = ?2",
        params![data, id.to_string()],
    )?;

    Ok(())
}

pub fn list_entries(conn: &Connection) -> Result<Vec<Entry>, Box<dyn Error>> {
    let mut stmt = conn.prepare("SELECT * FROM entries")?;
    let entries_iter = stmt.query_map([], |row| {
        let uuid_value: String = row.get_unwrap(0);
        let uuid = Uuid::parse_str(uuid_value.as_str()).map_err(|e| CustomError::UUID(e));
        let id = uuid?.to_string();
        let entry_type = row.get_unwrap(1);
        let version = row.get_unwrap(2);
        let meta_string: String = row.get_unwrap(3);
        let meta = serde_json::from_str(&meta_string).map_err(|e| CustomError::Json(e))?;

        let data_string: String = row.get_unwrap(4);
        let data = serde_json::from_str(data_string.as_str()).map_err(|e| CustomError::Json(e))?;

        Ok(Entry {
            id: id,
            entry_type: entry_type,
            data: data,
            meta: meta,
            version: version,
        })
    })?;
    // let entries_vec: Vec<Result<Entry, rusqlite::Error>> = entries_iter.collect();
    let (entries_vec, errors): (Vec<_>, Vec<_>) = entries_iter
        .map(|entry| entry.map_err(|e| CustomError::SQLite(e)))
        .partition(Result::is_ok);

    for result in errors {
        if let Err(err) = result {
            return Err(Box::new(err));
        }
    }
    let entries: Vec<Entry> = entries_vec.into_iter().map(Result::unwrap).collect();
    return Ok(entries);
}
