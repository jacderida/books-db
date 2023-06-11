mod books;
mod db;
mod error;
mod isbn_db;
mod models;

use books::BookRepository;
use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Help, Report, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Editor};
use isbn_db::IsbnDbRepository;
use models::AddBookModel;
use std::path::PathBuf;

const ISBNDB_URL: &str = "https://api2.isbndb.com";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Provide a custom directory for database storage
    #[arg(short, long, value_name = "DIR", global(true))]
    storage_path: Option<PathBuf>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create the database schema
    Init,
    /// Get the ISBNdb record for a book
    ///
    /// This will print the record for the book on the ISBNdb without saving it to the local
    /// database.
    Get {
        /// The book's ISBN
        #[clap(name = "isbn")]
        isbn: String,
    },
    /// Add a book to the database
    Add {
        /// The book's ISBN
        #[clap(name = "isbn")]
        isbn: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let database_path = get_database_path(cli.storage_path)?;
    let isbn_db_key = match std::env::var("ISBNDB_KEY") {
        Ok(val) => val,
        Err(_) => {
            return Err(eyre!("Could not obtain a key for the ISBNdb database")
                .suggestion("Please set the ISBNDB_KEY variable to your key"));
        }
    };

    let result = match cli.command {
        Some(Commands::Init) => {
            db::init_db(database_path)?;
            Ok(())
        }
        Some(Commands::Get { isbn }) => {
            let isbn_repo = IsbnDbRepository::new(ISBNDB_URL, &isbn_db_key);
            let book = isbn_repo.get_book_by_isbn(&isbn).await?;
            book.print();
            Ok(())
        }
        Some(Commands::Add { isbn }) => {
            let isbn_repo = IsbnDbRepository::new(ISBNDB_URL, &isbn_db_key);
            let book = isbn_repo.get_book_by_isbn(&isbn).await?;
            let mut model = AddBookModel::from(book);
            println!("Retrieved book with ISBN {isbn}");
            model.print();
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Edit details before saving?")
                .interact()
                .unwrap()
            {
                let to_edit = model.to_editor();
                if let Some(edited) = Editor::new().edit(&to_edit).unwrap() {
                    model = edited.parse()?;
                }
            }

            let book_repo = BookRepository::new(database_path);
            book_repo.add_book(model)?;
            println!("Saved book to the database.");
            Ok(())
        }
        None => {
            println!("No command provided. Please use --help to see a list of available commands.");
            Ok(())
        }
    };
    result
}

fn get_database_path(storage_path: Option<PathBuf>) -> Result<PathBuf> {
    let data_dir_path =
        dirs_next::data_dir().ok_or_else(|| eyre!("Unable to retrieve data directory"))?;
    let storage_path = storage_path.unwrap_or(data_dir_path.join("books-db"));
    std::fs::create_dir_all(storage_path.clone())?;
    let database_path = storage_path.join("books.db");
    Ok(database_path)
}
