use crate::error::{Error, Result};
use crate::models::AddBookModel;

use std::convert::TryFrom;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Author {
    pub id: u32,
    pub forename: String,
    pub surname: String,
}

#[derive(Debug)]
pub struct Publisher {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct Book {
    pub id: u32,
    pub authors: Vec<Author>,
    pub publisher: Publisher,
    pub title: String,
    pub edition: String,
    pub date_published: String,
    pub original_date_published: Option<String>,
    pub price: Option<f64>,
    pub binding: String,
    pub isbn: String,
    pub pages: u32,
    pub owned: bool,
}

impl TryFrom<AddBookModel> for Book {
    type Error = Error;

    fn try_from(item: AddBookModel) -> Result<Self, Self::Error> {
        let authors = item
            .authors
            .split(';')
            .map(|s| {
                let mut split = s.split(',');
                let surname = split.next().unwrap().trim();
                let forename = split.next().unwrap().trim();
                Author {
                    id: 0,
                    forename: forename.to_string(),
                    surname: surname.to_string(),
                }
            })
            .collect();
        Ok(Book {
            id: 0,
            authors,
            publisher: Publisher {
                id: 0,
                name: item.publisher,
            },
            title: item.title,
            edition: item.edition,
            date_published: item.date_published,
            original_date_published: item.original_date_published,
            price: item.price,
            binding: item.binding,
            isbn: item.isbn,
            pages: item.pages,
            owned: item.owned,
        })
    }
}

pub struct BookRepository {
    pub storage_path: PathBuf,
}

impl BookRepository {
    pub fn new(storage_path: PathBuf) -> BookRepository {
        BookRepository { storage_path }
    }

    pub fn add_book(&self, model: AddBookModel) -> Result<Book> {
        let mut book = Book::try_from(model)?;
        book.publisher.id = crate::db::save_publisher(self.storage_path.clone(), &book.publisher)?;
        for mut author in book.authors.iter_mut() {
            author.id = crate::db::save_author(self.storage_path.clone(), author)?;
        }
        book.id = crate::db::save_book(self.storage_path.clone(), &book)?;
        Ok(book)
    }

    #[allow(dead_code)]
    pub fn get_by_id(&self, id: u32) -> Result<Book> {
        let book = crate::db::get_book(self.storage_path.clone(), id)?;
        Ok(book)
    }
}

#[cfg(test)]
mod test {
    use super::{Book, BookRepository};
    use crate::db::init_db;
    use crate::models::AddBookModel;
    use assert_fs::prelude::*;
    use color_eyre::Result;

    #[test]
    fn try_from_should_convert_the_add_book_model_to_a_book() -> Result<()> {
        let model = AddBookModel {
            authors: "Reeve, Simon".to_string(),
            publisher: "Carlton Publishing Group".to_string(),
            title: "The New Jackals: Osama Bin Laden and the Future of Terrorism".to_string(),
            edition: "2nd".to_string(),
            date_published: "2001".to_string(),
            original_date_published: Some("1999".to_string()),
            price: Some(20.0),
            binding: "Paperback".to_string(),
            isbn: "9780233050485".to_string(),
            pages: 352,
            owned: true,
        };

        let book = Book::try_from(model)?;
        // This conversion will occur before the book is created, so an initial ID of 0 is
        // assigned.
        assert_eq!(book.id, 0);
        // At this point it is not known whether the publisher already exists in the database.
        // Therefore they are initially assigned an ID of 0. Before the book is saved
        // the publisher will be saved and assigned an ID, either new or existing.
        assert_eq!(book.publisher.id, 0);
        assert_eq!(book.publisher.name, "Carlton Publishing Group");
        assert_eq!(book.authors.len(), 1);
        // The same applies to the authors.
        assert_eq!(book.authors[0].id, 0);
        assert_eq!(book.authors[0].forename, "Simon");
        assert_eq!(book.authors[0].surname, "Reeve");
        assert_eq!(
            book.title,
            "The New Jackals: Osama Bin Laden and the Future of Terrorism"
        );
        assert_eq!(book.edition, "2nd");
        assert_eq!(book.date_published, "2001");
        assert_eq!(book.original_date_published, Some("1999".to_string()));
        assert_eq!(book.price, Some(20.0));
        assert_eq!(book.binding, "Paperback");
        assert_eq!(book.isbn, "9780233050485");
        assert_eq!(book.pages, 352);
        assert!(book.owned);
        Ok(())
    }

    #[test]
    fn try_from_should_convert_add_book_model_with_multiple_authors_to_a_book() -> Result<()> {
        let model = AddBookModel {
            authors: "Dwyer, Jim; Murphy, Deidre; Tyre, Peg; Kocieniewski, David".to_string(),
            publisher: "Crown".to_string(),
            title: "Two Seconds Under the World:Terror Comes to America-The Conspiracy Behind the World Trade Center Bombing".to_string(),
            edition: "1st".to_string(),
            date_published: "1997".to_string(),
            original_date_published: None,
            price: Some(20.0),
            binding: "Hardcover".to_string(),
            isbn: "9780517597675".to_string(),
            pages: 322,
            owned: true,
        };

        let book = Book::try_from(model)?;
        assert_eq!(book.id, 0);
        assert_eq!(book.publisher.id, 0);
        assert_eq!(book.publisher.name, "Crown");
        assert_eq!(book.authors.len(), 4);
        assert_eq!(book.authors[0].id, 0);
        assert_eq!(book.authors[0].forename, "Jim");
        assert_eq!(book.authors[0].surname, "Dwyer");
        assert_eq!(book.authors[1].id, 0);
        assert_eq!(book.authors[1].forename, "Deidre");
        assert_eq!(book.authors[1].surname, "Murphy");
        assert_eq!(book.authors[2].id, 0);
        assert_eq!(book.authors[2].forename, "Peg");
        assert_eq!(book.authors[2].surname, "Tyre");
        assert_eq!(book.authors[3].id, 0);
        assert_eq!(book.authors[3].forename, "David");
        assert_eq!(book.authors[3].surname, "Kocieniewski");
        assert_eq!(
            book.title,
            "Two Seconds Under the World:Terror Comes to America-The Conspiracy Behind the World Trade Center Bombing"
        );
        assert_eq!(book.edition, "1st");
        assert_eq!(book.date_published, "1997");
        assert_eq!(book.original_date_published, None);
        assert_eq!(book.price, Some(20.0));
        assert_eq!(book.binding, "Hardcover");
        assert_eq!(book.isbn, "9780517597675");
        assert_eq!(book.pages, 322);
        assert!(book.owned);
        Ok(())
    }

    #[test]
    fn add_book_should_save_book_authors_and_publisher() -> Result<()> {
        let storage_dir = assert_fs::TempDir::new().unwrap();
        let books_db_file = storage_dir.child("books.db");
        init_db(books_db_file.to_path_buf())?;

        let model = AddBookModel {
            authors: "Reeve, Simon".to_string(),
            publisher: "Carlton Publishing Group".to_string(),
            title: "The New Jackals: Osama Bin Laden and the Future of Terrorism".to_string(),
            edition: "2nd".to_string(),
            date_published: "2001".to_string(),
            original_date_published: Some("1999".to_string()),
            price: Some(20.0),
            binding: "Paperback".to_string(),
            isbn: "9780233050485".to_string(),
            pages: 352,
            owned: true,
        };
        let repository = BookRepository::new(books_db_file.to_path_buf());

        let book = repository.add_book(model)?;

        assert!(book.publisher.id != 0);
        assert!(book.authors[0].id != 0);
        assert!(book.id != 0);

        let id = book.id;
        let book = repository.get_by_id(id)?;

        assert_eq!(
            book.title,
            "The New Jackals: Osama Bin Laden and the Future of Terrorism"
        );
        assert_eq!(book.publisher.name, "Carlton Publishing Group");
        assert_eq!(book.authors[0].surname, "Reeve");
        assert_eq!(book.authors[0].forename, "Simon");
        assert_eq!(book.edition, "2nd");
        assert_eq!(book.date_published, "2001");
        assert_eq!(book.original_date_published, Some("1999".to_string()));
        assert_eq!(book.price, Some(20.0));
        assert_eq!(book.binding, "Paperback");
        assert_eq!(book.isbn, "9780233050485");
        assert_eq!(book.pages, 352);
        assert!(book.owned);

        Ok(())
    }

    #[test]
    fn add_book_should_save_book_with_multiple_authors() -> Result<()> {
        let storage_dir = assert_fs::TempDir::new().unwrap();
        let books_db_file = storage_dir.child("books.db");
        init_db(books_db_file.to_path_buf())?;

        let model = AddBookModel {
            authors: "Dwyer, Jim; Murphy, Deidre; Tyre, Peg; Kocieniewski, David".to_string(),
            publisher: "Crown".to_string(),
            title: "Two Seconds Under the World:Terror Comes to America-The Conspiracy Behind the World Trade Center Bombing".to_string(),
            edition: "1st".to_string(),
            date_published: "1997".to_string(),
            original_date_published: None,
            price: Some(20.0),
            binding: "Hardcover".to_string(),
            isbn: "9780517597675".to_string(),
            pages: 322,
            owned: true,
        };
        let repository = BookRepository::new(books_db_file.to_path_buf());

        let book = repository.add_book(model)?;

        assert!(book.publisher.id != 0);
        assert!(book.authors.iter().all(|a| a.id != 0));
        assert!(book.id != 0);

        let id = book.id;
        let book = repository.get_by_id(id)?;

        assert_eq!(
            book.title,
            "Two Seconds Under the World:Terror Comes to America-The Conspiracy Behind the World Trade Center Bombing"
        );
        assert_eq!(book.publisher.name, "Crown");
        assert_eq!(book.authors[0].surname, "Dwyer");
        assert_eq!(book.authors[0].forename, "Jim");
        assert_eq!(book.authors[1].surname, "Murphy");
        assert_eq!(book.authors[1].forename, "Deidre");
        assert_eq!(book.authors[2].surname, "Tyre");
        assert_eq!(book.authors[2].forename, "Peg");
        assert_eq!(book.authors[3].surname, "Kocieniewski");
        assert_eq!(book.authors[3].forename, "David");
        assert_eq!(book.edition, "1st");
        assert_eq!(book.date_published, "1997");
        assert_eq!(book.original_date_published, None);
        assert_eq!(book.price, Some(20.0));
        assert_eq!(book.binding, "Hardcover");
        assert_eq!(book.isbn, "9780517597675");
        assert_eq!(book.pages, 322);
        assert!(book.owned);

        Ok(())
    }
}
