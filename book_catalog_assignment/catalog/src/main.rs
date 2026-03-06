use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let file = File::create(filename);
    let mut file = match file{
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating the file: {:?}", error)
        },
    };

    for book in books{
        let result = writeln!(file, "{}, {}, {}", book.title, book.author, book.year);
        match result{
            Ok(_) => {}
            Err(error) => {
                panic!("Problem writing on to file: {:?}", error)
            },
        }
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename);
    let file = match file{
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening file: {:?}", error)
        },
    };
    let reader = BufReader::new(file);
    let mut books:Vec<Book> = Vec::new();

    for line in reader.lines(){
        let line = match line{
            Ok(line) => line,
            Err(error) => {
                panic!("Problem reading file: {:?}", error)
            },
        };
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3{
            let year = match parts[2].parse::<u16>(){
                Ok(year) => year,
                Err(error) => {
                    panic!("Problem parsing year: {:?}", error)
                },
            };
            let book = Book{
                title: parts[0].to_string(),
                author: parts[1].to_string(),
                year: year,
            };
            books.push(book);
        }
    }
    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}