use assert_cmd::Command;
use assert_fs::prelude::*;
use rusqlite::Connection;

#[test]
fn init_should_create_the_database() {
    let storage_dir = assert_fs::TempDir::new().unwrap();
    let books_db_file = storage_dir.child("books.db");

    let mut cmd = Command::cargo_bin("books").unwrap();
    cmd.arg("init")
        .arg("--storage-path")
        .arg(storage_dir.path().to_str().unwrap())
        .assert()
        .success();

    books_db_file.assert(predicates::path::is_file());
}

#[test]
fn init_should_create_the_publishers_table() {
    let storage_dir = assert_fs::TempDir::new().unwrap();
    let books_db_file = storage_dir.child("books.db");

    let mut cmd = Command::cargo_bin("books").unwrap();
    cmd.arg("init")
        .arg("--storage-path")
        .arg(storage_dir.path().to_str().unwrap())
        .assert()
        .success();

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
fn init_should_create_the_authors_table() {
    let storage_dir = assert_fs::TempDir::new().unwrap();
    let books_db_file = storage_dir.child("books.db");

    let mut cmd = Command::cargo_bin("books").unwrap();
    cmd.arg("init")
        .arg("--storage-path")
        .arg(storage_dir.path().to_str().unwrap())
        .assert()
        .success();

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
fn init_should_create_the_books_table() {
    let storage_dir = assert_fs::TempDir::new().unwrap();
    let books_db_file = storage_dir.child("books.db");

    let mut cmd = Command::cargo_bin("books").unwrap();
    cmd.arg("init")
        .arg("--storage-path")
        .arg(storage_dir.path().to_str().unwrap())
        .assert()
        .success();

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
fn init_should_create_the_books_authors_table() {
    let storage_dir = assert_fs::TempDir::new().unwrap();
    let books_db_file = storage_dir.child("books.db");

    let mut cmd = Command::cargo_bin("books").unwrap();
    cmd.arg("init")
        .arg("--storage-path")
        .arg(storage_dir.path().to_str().unwrap())
        .assert()
        .success();

    let conn = Connection::open(books_db_file.path()).unwrap();
    let mut statement = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
        .unwrap();
    let rows = statement
        .query_map(&["books_authors"], |row| row.get::<_, String>(0))
        .unwrap();
    assert!(rows.count() > 0)
}
