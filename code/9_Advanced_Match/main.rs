// Topic: Advanced match

// Requirements:
// * Print out a list of books and their information in a library
// * Books can be Fiction, Non-Fiction, and Reference
// * Fiction and Non-Fiction books include the author's name
// * All books include the title and publication year

// Notes:
// * Use an enum for the books with data associated with each variant
// * Create one of each book type and place them into a vector
// * Use a match expression while iterating the vector to print the book info

// * I personally added the Option<T> to merge topic exercises, so feel free to implement that  

//YOUR VERSION----------------------------------------------------------------------------------------------------------------

fn your_main () {
}

// MY VERSION------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct Book {
    title: String, 
    author: Option<String>,
    year: u64
}

#[derive(Debug)]
enum Category {
    Fiction(Book),
    NonFiction(Book),
    Reference(Book)
}

fn searching (library: Vec<Category>) {
    for book in library {
        match book {
            Category::Fiction(Book{title,author,year}) => println!("Title: {}, By: {:?}, Year: {}",title,author,year),
            Category::NonFiction(Book{title,author,year}) => println!("Title: {}, By: {:?}, Year: {}",title,author,year),
            Category::Reference(Book{title,author,year}) => println!("Title: {}, By: {:?}, Year: {}",title,author,year)

        }
    }
}

fn my_main () {

    let mut library: Vec<Category> = Vec::new();
    
    library.push(Category::Fiction(Book{ 
        title: "Foundation".to_owned(), 
        author: Some(String::from("Asimov")), 
        year: 1951 
    }));
    library.push(Category::NonFiction(Book{ 
        title: "Nineteen Eighty-Four".to_owned(), 
        author: Some(String::from("Orwell")),   
        year: 1949 
    }));
    library.push(Category::Reference(Book{ 
        title: "A Game Of Thrones".to_owned(), 
        author: None, 
        year: 1996 
    }));
    
    searching(library)

}

//AI------------------------------------------------------------------------------------------------------------------------

enum BookAI {
    FictionAI { title: String, author: String, year: u16 },
    NonFictionAI { title: String, author: String, year: u16 },
    ReferenceAI { title: String, year: u16 },
}

fn ai_main() {
    let books = vec![
        BookAI::FictionAI {
            title: String::from("The Great Gatsby"),
            author: String::from("F. Scott Fitzgerald"),
            year: 1925,
        },
        BookAI::NonFictionAI {
            title: String::from("Sapiens: A Brief History of Humankind"),
            author: String::from("Yuval Noah Harari"),
            year: 2011,
        },
        BookAI::ReferenceAI {
            title: String::from("Oxford English Dictionary"),
            year: 1884,
        },
    ];

    for book in books {
        match book {
            BookAI::FictionAI { title, author, year } => {
                println!("Fiction: '{}', by {}, published in {}", title, author, year);
            }
            BookAI::NonFictionAI { title, author, year } => {
                println!("Non-Fiction: '{}', by {}, published in {}", title, author, year);
            }
            BookAI::ReferenceAI { title, year } => {
                println!("Reference: '{}', published in {}", title, year);
            }
        }
    }
}

//MAIN-----------------------------------------------------------------------------------------------------------------------
fn main () {
    println!("\nYou:");
    your_main();
    println!("\nMe:");
    my_main();
    println!("\nAI:");
    ai_main();
    println!("");
}

/* 
Personal Comment post verification:

*/