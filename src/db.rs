use crate::books::{Author, Book, Publisher};
use crate::error::{Error, Result};
use rusqlite::{Connection, Result as RusqliteResult};
use std::path::PathBuf;

pub fn init_db(database_path: PathBuf) -> Result<()> {
    let conn = Connection::open(database_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS publishers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
         )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS authors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            forename TEXT NOT NULL,
            surname TEXT NOT NULL,
            UNIQUE(forename, surname)
         )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            publisher_id INTEGER,
            title TEXT NOT NULL,
            edition TEXT,
            date_published TEXT NOT NULL,
            original_date_published TEXT,
            price DECIMAL,
            binding TEXT NOT NULL,
            isbn TEXT NOT NULL,
            pages INTEGER NOT NULL DEFAULT 0,
            owned INTEGER NOT NULL DEFAULT 0,
            UNIQUE(title, edition),
            FOREIGN KEY (publisher_id) REFERENCES publishers(id)
         )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS books_authors (
            book_id INTEGER,
            author_id INTEGER,
            FOREIGN KEY (book_id) REFERENCES books(id),
            FOREIGN KEY (author_id) REFERENCES authors(id)
         )",
        [],
    )?;
    Ok(())
}

pub fn get_book(database_path: PathBuf, id: u32) -> Result<Book> {
    let conn = Connection::open(database_path)?;
    let mut book = match conn.query_row(
        "
        SELECT 
            books.id, books.title, books.edition, books.date_published,
            books.original_date_published, books.price, books.binding, 
            books.isbn, books.pages, books.owned, 
            publishers.id, publishers.name
        FROM books 
        LEFT JOIN publishers ON books.publisher_id = publishers.id
        WHERE books.id = ?1
    ",
        [id],
        |row| {
            let book_id: u32 = row.get(0)?;
            let title: String = row.get(1)?;
            let edition: String = row.get(2)?;
            let date_published: String = row.get(3)?;
            let original_date_published: Option<String> = row.get(4)?;
            let price: Option<f64> = row.get(5)?;
            let binding: String = row.get(6)?;
            let isbn: String = row.get(7)?;
            let pages: u32 = row.get(8)?;
            let owned: bool = row.get(9)?;
            let publisher_id: u32 = row.get(10)?;
            let publisher_name: String = row.get(11)?;

            let publisher = Publisher {
                id: publisher_id,
                name: publisher_name,
            };

            Ok(Book {
                id: book_id,
                authors: vec![],
                publisher,
                title,
                edition,
                date_published,
                original_date_published,
                price,
                binding,
                isbn,
                pages,
                owned,
            })
        },
    ) {
        Ok(book) => book,
        Err(e) => return Err(Error::DatabaseError(e)),
    };

    let mut stmt = conn.prepare(
        "
        SELECT authors.id, authors.forename, authors.surname
        FROM authors
        JOIN books_authors ON authors.id = books_authors.author_id
        WHERE books_authors.book_id = ?1
    ",
    )?;
    let author_rows: RusqliteResult<_> = stmt.query_map([id], |row| {
        let id: u32 = row.get(0)?;
        let forename: String = row.get(1)?;
        let surname: String = row.get(2)?;
        let author = Author {
            id,
            forename,
            surname,
        };
        Ok(author)
    });

    for author_result in author_rows? {
        book.authors.push(author_result?);
    }

    Ok(book)
}

pub fn save_publisher(database_path: PathBuf, publisher: &Publisher) -> Result<u32> {
    let conn = Connection::open(database_path)?;
    conn.execute(
        "INSERT OR IGNORE INTO publishers (name) VALUES (?1)",
        [&publisher.name],
    )?;
    let id = conn.query_row(
        "SELECT id FROM publishers WHERE name = ?1",
        [&publisher.name],
        |row| row.get(0),
    )?;
    Ok(id)
}

pub fn save_author(database_path: PathBuf, author: &Author) -> Result<u32> {
    let conn = Connection::open(database_path)?;
    conn.execute(
        "INSERT INTO authors (forename, surname) VALUES (?1, ?2)",
        [&author.forename, &author.surname],
    )?;
    let id = conn.query_row(
        "SELECT id FROM authors WHERE forename = ?1 AND surname = ?2",
        [&author.forename, &author.surname],
        |row| row.get(0),
    )?;
    Ok(id)
}

pub fn save_book(database_path: PathBuf, book: &Book) -> Result<u32> {
    let conn = Connection::open(database_path)?;
    conn.execute(
        "INSERT INTO books (
            publisher_id, title, edition,
            date_published, original_date_published, price,
            binding, isbn, pages, owned
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        (
            book.publisher.id,
            &book.title,
            &book.edition,
            &book.date_published,
            &book.original_date_published,
            &book.price,
            &book.binding,
            &book.isbn,
            &book.pages,
            &book.owned,
        ),
    )?;
    let id = conn.query_row(
        "SELECT id FROM books WHERE title = ?1 AND edition = ?2",
        [&book.title, &book.edition],
        |row| row.get(0),
    )?;
    for author in book.authors.iter() {
        conn.execute(
            "INSERT INTO books_authors (book_id, author_id) VALUES (?1, ?2)",
            (id, author.id),
        )?;
    }
    Ok(id)
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_fs::prelude::*;
    use rusqlite::Connection;

    #[test]
    fn init_db_should_create_the_database() {
        let storage_dir = assert_fs::TempDir::new().unwrap();
        let books_db_file = storage_dir.child("books.db");

        init_db(books_db_file.to_path_buf()).unwrap();

        books_db_file.assert(predicates::path::is_file());
    }

    #[test]
    fn init_db_should_create_the_publishers_table() {
        let storage_dir = assert_fs::TempDir::new().unwrap();
        let books_db_file = storage_dir.child("books.db");

        init_db(books_db_file.to_path_buf()).unwrap();

        let conn = Connection::open(books_db_file.path()).unwrap();
        let mut statement = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
            .unwrap();
        let rows = statement
            .query_map(&["publishers"], |row| row.get::<_, String>(0))
            .unwrap();
        assert!(rows.count() > 0)
    }

    #[test]
    fn init_db_should_create_the_authors_table() {
        let storage_dir = assert_fs::TempDir::new().unwrap();
        let books_db_file = storage_dir.child("books.db");

        init_db(books_db_file.to_path_buf()).unwrap();

        let conn = Connection::open(books_db_file.path()).unwrap();
        let mut statement = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
            .unwrap();
        let rows = statement
            .query_map(&["authors"], |row| row.get::<_, String>(0))
            .unwrap();
        assert!(rows.count() > 0)
    }

    #[test]
    fn init_db_should_create_the_books_table() {
        let storage_dir = assert_fs::TempDir::new().unwrap();
        let books_db_file = storage_dir.child("books.db");

        init_db(books_db_file.to_path_buf()).unwrap();

        let conn = Connection::open(books_db_file.path()).unwrap();
        let mut statement = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
            .unwrap();
        let rows = statement
            .query_map(&["books"], |row| row.get::<_, String>(0))
            .unwrap();
        assert!(rows.count() > 0)
    }

    #[test]
    fn init_db_should_create_the_books_authors_table() {
        let storage_dir = assert_fs::TempDir::new().unwrap();
        let books_db_file = storage_dir.child("books.db");

        init_db(books_db_file.to_path_buf()).unwrap();

        let conn = Connection::open(books_db_file.path()).unwrap();
        let mut statement = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
            .unwrap();
        let rows = statement
            .query_map(&["books_authors"], |row| row.get::<_, String>(0))
            .unwrap();
        assert!(rows.count() > 0)
    }
}
