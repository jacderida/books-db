use crate::error::Error;
use crate::isbn_db::IsbnDbBook;
use std::str::FromStr;

#[derive(Debug)]
pub struct AddBookModel {
    pub authors: String,
    pub publisher: String,
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

impl From<IsbnDbBook> for AddBookModel {
    fn from(item: IsbnDbBook) -> Self {
        let authors = item.authors.join("; ");
        let original_date_published = if item.edition == "1st" {
            Some(item.date_published.clone())
        } else {
            None
        };
        AddBookModel {
            authors,
            publisher: item.publisher,
            title: item.title_long,
            edition: item.edition,
            date_published: item.date_published,
            original_date_published,
            price: None,
            binding: item.binding,
            isbn: item.isbn13,
            pages: item.pages,
            owned: true,
        }
    }
}

impl FromStr for AddBookModel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut authors = None;
        let mut publisher = None;
        let mut title = None;
        let mut edition = None;
        let mut date_published = None;
        let mut original_date_published = None;
        let mut price = None;
        let mut binding = None;
        let mut isbn = None;
        let mut pages = None;
        let mut owned = None;

        for line in s.lines() {
            let mut parts = line.splitn(2, ':');
            let key = parts.next();
            let value = parts.next().unwrap_or("").trim();

            match key {
                Some("Author(s)") => authors = Some(value.to_string()),
                Some("Publisher") => publisher = Some(value.to_string()),
                Some("Title") => title = Some(value.to_string()),
                Some("Edition") => edition = Some(value.to_string()),
                Some("Date Published") => date_published = Some(value.to_string()),
                Some("Original Date Published") => {
                    original_date_published = if value.is_empty() {
                        None
                    } else {
                        Some(value.to_string())
                    }
                }
                Some("Price") => {
                    price = if value.is_empty() {
                        None
                    } else {
                        Some(value.parse().map_err(|_| {
                            Error::ParseError("Could not parse price field".to_string())
                        })?)
                    }
                }
                Some("Binding") => binding = Some(value.to_string()),
                Some("ISBN") => isbn = Some(value.to_string()),
                Some("Pages") => {
                    pages = Some(value.parse().map_err(|_| {
                        Error::ParseError("Could not parse pages field".to_string())
                    })?)
                }
                Some("Owned") => {
                    owned = Some(value.parse().map_err(|_| {
                        Error::ParseError("Could not parse pages field".to_string())
                    })?)
                }
                _ => {
                    return Err(Error::ParseError(format!(
                        "Could not parse {} AddBookModel from string",
                        key.unwrap()
                    )))
                }
            }
        }

        Ok(Self {
            authors: authors.ok_or_else(|| Error::ParseError("Missing authors".to_string()))?,
            publisher: publisher
                .ok_or_else(|| Error::ParseError("Missing publisher".to_string()))?,
            title: title.ok_or_else(|| Error::ParseError("Missing title".to_string()))?,
            edition: edition.ok_or_else(|| Error::ParseError("Missing edition".to_string()))?,
            date_published: date_published
                .ok_or_else(|| Error::ParseError("Missing date_published".to_string()))?,
            original_date_published,
            price,
            binding: binding.ok_or_else(|| Error::ParseError("Missing binding".to_string()))?,
            isbn: isbn.ok_or_else(|| Error::ParseError("Missing isbn".to_string()))?,
            pages: pages.ok_or_else(|| Error::ParseError("Missing pages".to_string()))?,
            owned: owned.ok_or_else(|| Error::ParseError("Missing owned".to_string()))?,
        })
    }
}

impl AddBookModel {
    pub fn newline(&self) -> &'static str {
        if cfg!(windows) {
            "\r\n"
        } else {
            "\n"
        }
    }

    pub fn print(&self) {
        println!("Title: {}", self.title);
        println!("Author(s): {}", self.authors);
        println!("Edition: {}", self.edition);
        println!("Date Published: {}", self.date_published);
        println!("Binding: {}", self.binding);
        println!("ISBN: {}", self.isbn);
        println!("Pages: {}", self.pages);
        println!("Owned: {}", self.owned);
    }

    pub fn to_editor(&self) -> String {
        let newline = self.newline();
        let original_date_published = self
            .original_date_published
            .as_ref()
            .map_or("".to_string(), |date| date.to_string());
        let price = self.price.map_or(String::new(), |price| price.to_string());
        format!(
            "Author(s): {}{newline}Publisher: {}{newline}Title: {}{newline}Edition: {}{newline}Date Published: {}{newline}Original Date Published: {}{newline}Price: {}{newline}Binding: {}{newline}ISBN: {}{newline}Pages: {}{newline}Owned: {}",
            self.authors,
            self.publisher,
            self.title,
            self.edition,
            self.date_published,
            original_date_published,
            price,
            self.binding,
            self.isbn,
            self.pages,
            self.owned,
            newline = newline
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::isbn_db::IsbnDbBook;
    use color_eyre::Result;

    #[test]
    fn from_should_convert_the_isbn_record_to_an_add_book_model() -> Result<()> {
        let isbn_book = IsbnDbBook {
            publisher: "Carlton Publishing Group".to_string(),
            authors: vec!["Reeve, Simon".to_string()],
            language: "en".to_string(),
            image_url: "https://images.isbndb.com/covers/04/85/9780233050485.jpg".to_string(),
            title_long: "The New Jackals: Osama Bin Laden and the Future of Terrorism".to_string(),
            edition: "2nd".to_string(),
            dimensions: "Height: 7.71652 Inches.to_string(), Length: 5.07873 Inches, Weight: 0.661386786 Pounds, Width: 0.7874 Inches".to_string(),
            pages: 352,
            date_published: "2001".to_string(),
            title: "The New Jackals".to_string(),
            isbn13: "9780233050485".to_string(),
            msrp: 17.75,
            binding: "Paperback".to_string(),
            isbn: "0233050485".to_string(),
            isbn10: "0233050485".to_string(),
            subjects: None,
            synopsis: None,
        };

        let model = AddBookModel::from(isbn_book);
        assert_eq!(model.publisher, "Carlton Publishing Group");
        assert_eq!(model.edition, "2nd");
        assert_eq!(model.pages, 352);
        assert_eq!(model.date_published, "2001");
        assert_eq!(model.original_date_published, None);
        assert_eq!(model.price, None);
        assert_eq!(
            model.title,
            "The New Jackals: Osama Bin Laden and the Future of Terrorism"
        );
        assert_eq!(model.isbn, "9780233050485");
        assert_eq!(model.binding, "Paperback");
        assert_eq!(model.authors, "Reeve, Simon");
        assert!(model.owned);
        Ok(())
    }

    #[test]
    fn from_should_convert_the_isbn_record_with_multiple_authors_to_an_add_book_model() -> Result<()>
    {
        let isbn_book = IsbnDbBook {
            publisher: "Crown".to_string(),
            authors: vec![
                "Dwyer, Jim".to_string(),
                "Murphy, Deidre".to_string(),
                "Tyre, Peg".to_string(),
                "Kocieniewski, David".to_string(),
            ],
            language: "en".to_string(),
            image_url: "https://images.isbndb.com/covers/76/75/9780517597675.jpg".to_string(),
            title_long: "Two Seconds Under the World:Terror Comes to America-The Conspiracy Behind the World Trade Center Bombing".to_string(),
            edition: "1st".to_string(),
            dimensions: "Height: 9.5 Inches, Length: 6.25 Inches, Weight: 1.4 Pounds, Width: 1 Inches".to_string(),
            pages: 322,
            date_published: "1997".to_string(),
            title: "Two Seconds Under the World:Terror Comes to America-The Conspiracy Behind the World Trade Center Bombing".to_string(),
            isbn13: "9780517597675".to_string(),
            msrp: 24_f32,
            binding: "Hardcover".to_string(),
            isbn: "0517597675".to_string(),
            isbn10: "0517597675".to_string(),
            subjects:
                Some(vec![
                    "World Trade Center Bombing, New York, N.Y., 1993".to_string(),
                    "Terrorism".to_string(),
                    "Terrorism--New York (State)--New York".to_string(),
                    "HV6432 .T88 1994".to_string(),
                    "364.1/09747/1".to_string(),
                ]),
            synopsis: Some("Text And Accompanying Photographs Present The Story Of The Bombing Of The World Trade Center By Islamic Fundamentalist Terrorists. Jim Dwyer ... [et Al.]. Includes Bibliographical References And Index.".to_string())
        };

        let model = AddBookModel::from(isbn_book);
        assert_eq!(model.publisher, "Crown");
        assert_eq!(model.edition, "1st");
        assert_eq!(model.pages, 322);
        assert_eq!(model.date_published, "1997");
        assert_eq!(model.price, None);
        assert_eq!(
            model.title,
            "Two Seconds Under the World:Terror Comes to America-The Conspiracy Behind the World Trade Center Bombing"
        );
        assert_eq!(model.isbn, "9780517597675");
        assert_eq!(model.binding, "Hardcover");
        assert_eq!(
            model.authors,
            "Dwyer, Jim; Murphy, Deidre; Tyre, Peg; Kocieniewski, David"
        );
        assert!(model.owned);
        // If the book is first edition, the original date should be set automatically.
        assert_eq!(model.original_date_published, Some("1997".to_string()));
        Ok(())
    }

    #[test]
    fn to_editor_should_represent_model_as_an_editable_string() -> Result<()> {
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
        let edit = model.to_editor();
        let newline = model.newline();
        let expected = format!(
            "Author(s): Reeve, Simon{nl}\
            Publisher: Carlton Publishing Group{nl}\
            Title: The New Jackals: Osama Bin Laden and the Future of Terrorism{nl}\
            Edition: 2nd{nl}\
            Date Published: 2001{nl}\
            Original Date Published: 1999{nl}\
            Price: 20{nl}\
            Binding: Paperback{nl}\
            ISBN: 9780233050485{nl}\
            Pages: 352{nl}\
            Owned: true",
            nl = newline
        );
        assert_eq!(edit, expected);
        Ok(())
    }

    #[test]
    fn parse_should_convert_an_edited_book_string_to_an_add_book_model() {
        let edited = "Author(s): Reeve, Simon\n\
         Publisher: Carlton Publishing Group\n\
         Title: The New Jackals: Osama Bin Laden and the Future of Terrorism\n\
         Edition: 2nd\n\
         Date Published: 2001\n\
         Original Date Published: 1999\n\
         Price: 20.0\n\
         Binding: Paperback\n\
         ISBN: 9780233050485\n\
         Pages: 352\n\
         Owned: true";

        let model: AddBookModel = edited.parse().unwrap();

        assert_eq!(model.authors, "Reeve, Simon");
        assert_eq!(model.publisher, "Carlton Publishing Group");
        assert_eq!(
            model.title,
            "The New Jackals: Osama Bin Laden and the Future of Terrorism"
        );
        assert_eq!(model.edition, "2nd");
        assert_eq!(model.date_published, "2001");
        assert_eq!(model.original_date_published, Some("1999".to_string()));
        assert_eq!(model.price, Some(20.0));
        assert_eq!(model.binding, "Paperback");
        assert_eq!(model.isbn, "9780233050485");
        assert_eq!(model.pages, 352);
        assert!(model.owned);
    }

    #[test]
    fn parse_should_convert_an_edited_book_string_with_optional_fields_missing_to_an_add_book_model(
    ) {
        let edited = "Author(s): Reeve, Simon\n\
         Publisher: Carlton Publishing Group\n\
         Title: The New Jackals: Osama Bin Laden and the Future of Terrorism\n\
         Edition: 2nd\n\
         Date Published: 2001\n\
         Binding: Paperback\n\
         ISBN: 9780233050485\n\
         Pages: 352\n\
         Owned: true";

        let model: AddBookModel = edited.parse().unwrap();

        assert_eq!(model.authors, "Reeve, Simon");
        assert_eq!(model.publisher, "Carlton Publishing Group");
        assert_eq!(
            model.title,
            "The New Jackals: Osama Bin Laden and the Future of Terrorism"
        );
        assert_eq!(model.edition, "2nd");
        assert_eq!(model.date_published, "2001");
        assert_eq!(model.original_date_published, None);
        assert_eq!(model.price, None);
        assert_eq!(model.binding, "Paperback");
        assert_eq!(model.isbn, "9780233050485");
        assert_eq!(model.pages, 352);
        assert!(model.owned);
    }

    #[test]
    fn parse_should_convert_an_edited_book_string_with_optional_fields_not_populated_to_an_add_book_model(
    ) {
        let edited = "Author(s): Reeve, Simon\n\
         Publisher: Carlton Publishing Group\n\
         Title: The New Jackals: Osama Bin Laden and the Future of Terrorism\n\
         Edition: 2nd\n\
         Date Published: 2001\n\
         Original Date Published:\n\
         Price: \n\
         Binding: Paperback\n\
         ISBN: 9780233050485\n\
         Pages: 352\n\
         Owned: true";

        let model: AddBookModel = edited.parse().unwrap();

        assert_eq!(model.authors, "Reeve, Simon");
        assert_eq!(model.publisher, "Carlton Publishing Group");
        assert_eq!(
            model.title,
            "The New Jackals: Osama Bin Laden and the Future of Terrorism"
        );
        assert_eq!(model.edition, "2nd");
        assert_eq!(model.date_published, "2001");
        assert_eq!(model.original_date_published, None);
        assert_eq!(model.price, None);
        assert_eq!(model.binding, "Paperback");
        assert_eq!(model.isbn, "9780233050485");
        assert_eq!(model.pages, 352);
        assert!(model.owned);
    }
}
