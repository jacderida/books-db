# books-db

This is a simple command line application for maintaining a collection of books. It uses [ISBNdb](https://isbndb.com) as a data source and [SQLite](https://www.sqlite.org/index.html) for storage.

## Setup

Obtain an API key from ISBNdb and set this using the `ISBNDB_KEY` environment variable.

Edit-based commands will use an external editor. Use the standard `EDITOR` or `VISUAL` environment variables to specify which editor to use.

Use the `init` command to create the database. On Linux, the file will be created at `~/.local/share/books-db/books.db`.

## Working with Books

### Get the ISBN Record

Use the `get` command with the ISBN to display the ISBNdb record for the book:
```
books get 9780517597675
```

This will print the record without saving it as a book in your local database.

### Add a Book to the Database

Use the `add` command with the ISBN to save a book to your database:
```
books add 9780517597675
```

Before the book is saved, you'll get an opportunity to edit any details.
