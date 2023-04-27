#![allow(unused)]

use std::fmt::{Debug, Display};

pub struct Library {
    books: Vec<Book>,
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

    fn print_books(&self) {
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

#[derive(PartialEq, Clone, Debug)]
pub struct Book {
    title: String,
    year: u16,
}
impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

fn main() {
    // This shows the desired behavior. Uncomment the code below and
    // implement the missing methods. You will need to update the
    // method signatures, including the "self" parameter!
    let library = Library::new();

    //println!("Our library is empty: {}", library.is_empty());
    //
    //library.add_book(Book::new("Lord of the Rings", 1954));
    //library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    //
    //library.print_books();
    //
    //match library.oldest_book() {
    //    Some(book) => println!("My oldest book is {book}"),
    //    None => println!("My library is empty!"),
    //}
    //
    //println!("Our library has {} books", library.len());
}
