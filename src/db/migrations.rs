use rusqlite::Connection;
use anyhow::Result;

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "PRAGMA foreign_keys = ON;
         PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;"
    )?;
    
    conn.execute_batch(include_str!("schema.sql"))?;
    
    Ok(())
}