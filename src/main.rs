fn main() {
    let b = Book::new(String::from("123"), String::from("me"), 6);

    let l = Library::from(b);

    l.list_available_books();

    let title =String::from("345");
    let b = l.find_book_by_title(&title);

    match b {
        Some(book) => println!("{}", book.get_info()),
        None => println!("Book not found")
    }
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Library{
        Library { books: Vec::new() }
    }

    fn from(book: Book) -> Library {
        let mut l = Library::new();
        l.add_book(book);
        l
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn list_available_books(&self) {
        for book in self.books.iter() {
            println!("{}", book.get_info());
        }
    }

    fn find_book_by_title(&self, title: &String) -> Option<&Book> {
        for book in self.books.iter() {
            if book.title == *title {
                return Some(book);
            }
        }
        None
    }
}

struct Book {
    title: String,
    author: String,
    pages: u32,
    is_available: bool,
}

impl Book {
    fn new(title: String, author: String, pages: u32) -> Book {
        Book {
            title,
            author,
            pages,
            is_available: true,
        }
    }

    fn borrow_book(&mut self) {
        if self.is_available == false {
            println!("Book is not available");
        } else {
            self.is_available = false;
        }
    }

    fn return_book(&mut self) {
        if self.is_available == true {
            println!("Book is already available");
        } else {
            self.is_available = true;
        }
    }

    fn get_info(&self) -> String {
        format!("'{}' by {}, {} pages", self.title, self.author, self.pages)
    }
}
