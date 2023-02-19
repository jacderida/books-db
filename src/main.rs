mod db;

use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Report, Result};
use std::path::PathBuf;

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
    /// Create the database schema.
    Init,
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let data_dir_path =
        dirs_next::data_dir().ok_or_else(|| eyre!("Unable to retrieve data directory"))?;
    let storage_path = cli.storage_path.unwrap_or(data_dir_path.join("books-db"));
    let database_path = storage_path.join("books.db");
    let result = match cli.command {
        Some(Commands::Init) => {
            db::init_db(database_path)?;
            Ok(())
        }
        None => {
            println!("No command provided. Please use --help to see a list of available commands.");
            Ok(())
        }
    };
    result
}
