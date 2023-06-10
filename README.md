# books-db

A simple command line application for maintaining a collection of books. It uses [ISBNdb](https://isbndb.com) as a data source and [SQLite](https://www.sqlite.org/index.html) for storage.

## Setup

Obtain an API key from ISBNdb and set this using the `ISBNDB_KEY` environment variable.

If there is a need to edit any details of a book, an external editor will be used for this. Set the standard `EDITOR` or `VISUAL` variables to specify the editor you want to use.

Use the `init` command to create the database. On Linux, the file will be created at `~/.local/share/books-db/books.db`.

## Adding a Book

Use the `add` command to add a book to your database, providing the ISBN:
```
books add 9780517597675
```

This will query the ISBNdb database and return the entry corresponding to the provided ISBN.

Before the book is saved, you'll get an opportunity to edit any details provided by ISBNdb you'd rather change or not have, and also such things as adding a price or indicating whether you own the book.
