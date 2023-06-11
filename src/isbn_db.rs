use color_eyre::Result;
use prettytable::{Cell, Row, Table};
use serde_json::Value;

const WRAP_LENGTH: usize = 80;

pub struct IsbnDbBook {
    pub publisher: String,
    pub language: String,
    pub image_url: String,
    pub title_long: String,
    pub edition: String,
    pub dimensions: String,
    pub pages: u32,
    pub date_published: String,
    pub authors: Vec<String>,
    pub title: String,
    pub isbn13: String,
    pub msrp: f32,
    pub binding: String,
    pub isbn: String,
    pub isbn10: String,
    pub subjects: Option<Vec<String>>,
    pub synopsis: Option<String>,
}

impl IsbnDbBook {
    pub fn print(&self) {
        let mut table = Table::new();
        let wrapped_title = textwrap::wrap(&self.title, WRAP_LENGTH).join("\n");
        table.add_row(Row::new(vec![
            Cell::new("Title"),
            Cell::new(&wrapped_title),
        ]));
        if self.title_long != self.title {
            let wrapped_long_title = textwrap::wrap(&self.title_long, 72).join("\n");
            table.add_row(Row::new(vec![
                Cell::new("Title (Long)"),
                Cell::new(&wrapped_long_title),
            ]));
        }
        table.add_row(Row::new(vec![
            Cell::new("Author(s)"),
            Cell::new(&self.authors.join("; ")),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Date Published"),
            Cell::new(&self.date_published),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Binding"),
            Cell::new(&self.binding),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Edition"),
            Cell::new(&self.edition),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Pages"),
            Cell::new(&self.pages.to_string()),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Publisher"),
            Cell::new(&self.publisher),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("Language"),
            Cell::new(&self.language),
        ]));

        let wrapped_subjects = textwrap::wrap(
            &self
                .subjects
                .as_ref()
                .map(|subjects| subjects.join(", "))
                .unwrap_or_else(|| "N/A".to_string()),
            WRAP_LENGTH,
        )
        .join("\n");
        table.add_row(Row::new(vec![
            Cell::new("Subjects"),
            Cell::new(&wrapped_subjects),
        ]));

        let wrapped_synopsis = textwrap::wrap(
            self.synopsis.as_ref().unwrap_or(&"N/A".to_string()),
            WRAP_LENGTH,
        )
        .join("\n");
        table.add_row(Row::new(vec![
            Cell::new("Synopsis"),
            Cell::new(&wrapped_synopsis),
        ]));
        table.add_row(Row::new(vec![
            Cell::new("MSRP"),
            Cell::new(&self.msrp.to_string()),
        ]));
        table.add_row(Row::new(vec![Cell::new("ISBN13"), Cell::new(&self.isbn13)]));
        table.add_row(Row::new(vec![Cell::new("ISBN"), Cell::new(&self.isbn)]));
        table.add_row(Row::new(vec![Cell::new("ISBN10"), Cell::new(&self.isbn10)]));
        table.add_row(Row::new(vec![
            Cell::new("Dimensions"),
            Cell::new(&self.dimensions),
        ]));
        table.printstd();
    }
}

pub struct IsbnDbRepository {
    pub base_url: String,
    pub rest_key: String,
}

impl IsbnDbRepository {
    pub fn new(base_url: &str, rest_key: &str) -> IsbnDbRepository {
        IsbnDbRepository {
            base_url: base_url.to_string(),
            rest_key: rest_key.to_string(),
        }
    }

    pub async fn get_book_by_isbn(&self, isbn: &str) -> Result<IsbnDbBook> {
        let url = format!("{}/book/{}", self.base_url, isbn);
        let resp = reqwest::Client::new()
            .get(&url)
            .header("accept", "application/json")
            .header("Authorization", self.rest_key.clone())
            .send()
            .await?
            .text()
            .await?;
        let v: Value = serde_json::from_str(&resp)?;
        Ok(IsbnDbBook {
            publisher: v["book"]["publisher"].as_str().unwrap().to_string(),
            language: v["book"]["language"].as_str().unwrap().to_string(),
            image_url: v["book"]["image"].as_str().unwrap().to_string(),
            title_long: v["book"]["title_long"].as_str().unwrap().to_string(),
            edition: v["book"]["edition"].as_str().unwrap().to_string(),
            dimensions: v["book"]["dimensions"].as_str().unwrap().to_string(),
            pages: v["book"]["pages"].as_i64().unwrap() as u32,
            date_published: v["book"]["date_published"].as_str().unwrap().to_string(),
            authors: v["book"]["authors"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect(),
            title: v["book"]["title"].as_str().unwrap().to_string(),
            isbn13: v["book"]["isbn13"].as_str().unwrap().to_string(),
            msrp: v["book"]["msrp"].as_f64().unwrap() as f32,
            binding: v["book"]["binding"].as_str().unwrap().to_string(),
            isbn: v["book"]["isbn"].as_str().unwrap().to_string(),
            isbn10: v["book"]["isbn10"].as_str().unwrap().to_string(),
            synopsis: v["book"]["synopsis"].as_str().map(String::from),
            subjects: v["book"]["subjects"].as_array().map(|arr| {
                arr.iter()
                    .filter_map(|x| x.as_str())
                    .map(String::from)
                    .collect()
            }),
        })
    }
}

#[cfg(test)]
mod test {
    use super::IsbnDbRepository;
    use color_eyre::Result;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn get_book_by_isbn_should_return_book_record() -> Result<()> {
        let isbn = "9780233050485";
        let server = MockServer::start();
        let response_body = std::fs::read_to_string(
            std::path::Path::new("resources").join("book_response_body.json"),
        )?;
        let latest_release_mock = server.mock(|when, then| {
            when.method(GET).path(format!("/book/{isbn}"));
            then.status(200)
                .header("server", "isbndb.com")
                .body(response_body);
        });

        let repository = IsbnDbRepository::new(&server.base_url(), "api_key");
        let book = repository.get_book_by_isbn(isbn).await?;

        assert_eq!(book.publisher, "Carlton Publishing Group");
        assert_eq!(book.language, "en");
        assert_eq!(
            book.image_url,
            "https://images.isbndb.com/covers/04/85/9780233050485.jpg"
        );
        assert_eq!(
            book.title_long,
            "The New Jackals: Osama Bin Laden and the Future of Terrorism"
        );
        assert_eq!(book.edition, "2nd");
        assert_eq!(book.dimensions, "Height: 7.71652 Inches, Length: 5.07873 Inches, Weight: 0.661386786 Pounds, Width: 0.7874 Inches");
        assert_eq!(book.pages, 352);
        assert_eq!(book.date_published, "2001");
        assert_eq!(
            book.title,
            "The New Jackals: Osama Bin Laden and the Future of Terrorism"
        );
        assert_eq!(book.isbn13, isbn);
        assert_eq!(book.msrp, 17.75);
        assert_eq!(book.binding, "Paperback");
        assert_eq!(book.isbn, "0233050485");
        assert_eq!(book.isbn10, "0233050485");
        assert_eq!(book.authors.len(), 1);
        assert_eq!(book.authors[0], "Reeve, Simon");
        assert_eq!(book.synopsis, None);
        assert_eq!(book.subjects, None);
        latest_release_mock.assert();
        Ok(())
    }

    #[tokio::test]
    async fn get_book_by_isbn_when_book_has_multiple_authors_and_optional_fields_should_return_book_record(
    ) -> Result<()> {
        let isbn = "9780517597675";
        let server = MockServer::start();
        let response_body = std::fs::read_to_string(
            std::path::Path::new("resources")
                .join("book_with_multiple_authors_and_optional_fields_response_body.json"),
        )?;
        let latest_release_mock = server.mock(|when, then| {
            when.method(GET).path(format!("/book/{isbn}"));
            then.status(200)
                .header("server", "isbndb.com")
                .body(response_body);
        });

        let repository = IsbnDbRepository::new(&server.base_url(), "api_key");
        let book = repository.get_book_by_isbn(isbn).await?;

        assert_eq!(book.publisher, "Crown");
        assert_eq!(book.synopsis, Some("Text And Accompanying Photographs Present The Story Of The Bombing Of The World Trade Center By Islamic Fundamentalist Terrorists. Jim Dwyer ... [et Al.]. Includes Bibliographical References And Index.".to_string()));
        assert_eq!(book.language, "en");
        assert_eq!(
            book.image_url,
            "https://images.isbndb.com/covers/76/75/9780517597675.jpg"
        );
        assert_eq!(
            book.title_long,
            "Two Seconds Under the World:Terror Comes to America-The Conspiracy Behind the World Trade Center Bombing"
        );
        assert_eq!(book.edition, "1st");
        assert_eq!(
            book.dimensions,
            "Height: 9.5 Inches, Length: 6.25 Inches, Weight: 1.4 Pounds, Width: 1 Inches"
        );
        assert_eq!(book.pages, 322);
        assert_eq!(book.date_published, "1997");
        assert_eq!(
            book.title,
            "Two Seconds Under the World:Terror Comes to America-The Conspiracy Behind the World Trade Center Bombing"
        );
        assert_eq!(book.isbn13, isbn);
        assert_eq!(book.msrp, 24_f32);
        assert_eq!(book.binding, "Hardcover");
        assert_eq!(book.isbn, "0517597675");
        assert_eq!(book.isbn10, "0517597675");
        assert_eq!(book.authors.len(), 4);
        assert_eq!(book.authors[0], "Dwyer, Jim");
        assert_eq!(book.authors[1], "Murphy, Deidre");
        assert_eq!(book.authors[2], "Tyre, Peg");
        assert_eq!(book.authors[3], "Kocieniewski, David");
        assert_eq!(book.synopsis, Some("Text And Accompanying Photographs Present The Story Of The Bombing Of The World Trade Center By Islamic Fundamentalist Terrorists. Jim Dwyer ... [et Al.]. Includes Bibliographical References And Index.".to_string()));
        assert_eq!(
            book.subjects,
            Some(vec![
                "World Trade Center Bombing, New York, N.Y., 1993".to_string(),
                "Terrorism".to_string(),
                "Terrorism--New York (State)--New York".to_string(),
                "HV6432 .T88 1994".to_string(),
                "364.1/09747/1".to_string(),
            ])
        );
        latest_release_mock.assert();
        Ok(())
    }
}
