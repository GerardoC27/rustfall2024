use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) -> Result<(), Error> {
    let mut file = File::create(filename)?;
    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year)?;
    }
    Ok(())
}

fn load_books(filename: &str) -> Result<Vec<Book>, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let book = Book {
                title: parts[0].to_string(),
                author: parts[1].to_string(),
                year: parts[2].parse().unwrap_or(0),
            };
            books.push(book);
        }
    }
    Ok(books)
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    match save_books(&books, "books.txt") {
        Ok(_) => println!("Books saved to file."),
        Err(e) => println!("Failed to save books: {}", e),
    }

    match load_books("books.txt") {
        Ok(loaded_books) => {
            println!("Loaded books:");
            for book in loaded_books {
                println!("{} by {}, published in {}", book.title, book.author, book.year);
            }
        }
        Err(e) => println!("Failed to load books: {}", e),
    }
}
