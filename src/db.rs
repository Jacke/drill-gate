use rusqlite::{ params, Connection, Result };

pub fn get_connection(db_name: &str) -> Result<Connection> {
    let conn = Connection::open(db_name)?;
    return Ok(conn);
}