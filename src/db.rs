use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn init_db(database_path: PathBuf) -> Result<()> {
    let conn = Connection::open(database_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS publishers (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
         )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS authors (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
         )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY,
            publisher_id INTEGER,
            title TEXT NOT NULL,
            edition TEXT,
            date_published TEXT NOT NULL,
            original_date_published TEXT,
            price DECIMAL,
            type TEXT NOT NULL,
            isbn TEXT NOT NULL,
            isbn13 TEXT,
            isbn10 TEXT,
            owned INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (publisher_id) REFERENCES publishers(publisher_id)
         )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS books_authors (
            book_id INTEGER,
            author_id INTEGER,
            FOREIGN KEY (book_id) REFERENCES books(book_id),
            FOREIGN KEY (author_id) REFERENCES authors(author_id)
         )",
        [],
    )?;
    Ok(())
}
