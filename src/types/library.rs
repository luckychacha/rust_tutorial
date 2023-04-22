#![allow(unused)]

use std::fmt::{Debug, Display};

pub struct Library {
    books: Vec<Book>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Book {
    title: String,
    year: u16,
}

impl Library {
    fn new() -> Self {
        Self { books: Vec::new() }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }

    fn list_books(&self) {
        self.books.iter().for_each(|item| {
            println!("{} {}", item.title, item.year);
        })
    }

    fn oldest_book(&self) -> Option<Book> {
        if self.books.is_empty() {
            return None;
        }

        let mut tmp: Option<Book> = None;
        self.books.iter().for_each(|item| match &tmp {
            Some(book) => {
                if book.year > item.year {
                    // book.title = item.title;
                    // book.year = item.year;
                    tmp = Some(item.clone());
                }
            }
            None => {
                tmp = Some(Book {
                    title: item.title.clone(),
                    year: item.year,
                })
            }
        });
        tmp
    }
}

impl Display for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "display: book count: {}", self.books.len())
    }
}

impl Debug for Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "debug: book count: {}", self.books.len())
    }
}

fn main() {
    let mut library = Library::new();
    library.add_book(Book {
        title: String::from("a"),
        year: 2021,
    });

    library.add_book(Book {
        title: String::from("b"),
        year: 2022,
    });

    println!("{:?}", library);
    library.list_books();
    println!("{:?}", library.oldest_book());
}
