//Farrel Ilham Octareswara Dava 2301956746
//Library System Information
 
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]

struct Book{ //Declare a struct that will be used for main
    title : String, 
    volume : i32
}

struct HashInput{
    hash : HashMap<String, Book> //HashMap used for declare a function by String
}

impl HashInput{

    fn new() -> Self{
        Self{
            hash : HashMap::new() 
        }
    }
    fn addbook(&mut self, book: Book){ //Input Data Book
        self.hash.insert(book.title.clone(), book);
    }

    fn dataBook(&self) -> Vec<&Book>{ //Book saved into Vector
        let mut books = vec![];
        for book in self.hash.values(){
            books.push(book) //InputData pushed by HashMap into books vector
        } books
    }

    fn updateBook(&mut self, title: &str, volume: i32) -> bool{ //Update book information by title of book
        match self.hash.get_mut(title){
            Some(book) => {
                book.volume = volume;
                true
            },
            None => false
        }
    }

    fn searchBook(&mut self, title: &str) -> bool{ //Search Enging function that match book title by String
        match self.hash.get_mut(title){
            Some(_inputData) => {
                true
            },
            None => false
        }
    }
}


fn Input() -> Option<String>{ // Input Function
    let mut hold = String::new();
    io::stdin().read_line(&mut hold).expect("Failed to entry data");
    let inputData = hold.trim().to_owned();
    if &inputData == "" {
        None
    } else {
        Some(inputData)
    }
}

fn inputVolume() -> Option<i32>{ //
    println!("Volume : ");
    loop{
    let vol = match Input(){
        Some(inputData) => inputData,
        None => return None
    };
    if &vol == "" {
        return None;
    }
    let inputParse : Result<i32, _> = vol.parse();
    match inputParse{
        Ok(volume) => return Some(volume), Err(_) => println!("Enter which volumes of the books")
        }
    }
}

fn Add_new_book(books: &mut HashInput){
    println!("Book title: ");
    let title = match Input(){
        Some(inputData) => inputData,
        None => return
    };
    let volume = match inputVolume(){
        Some(volume) => volume,
        None => return
    };
    let book = Book {title, volume};
    books.addbook(book);
    println!("New book added!")
}

fn Book_list_books(books: &HashInput){
    for book in books.dataBook(){
        println!("{:?}", book)
    }
}

fn Update_books(books: &mut HashInput){
    for book in books.dataBook(){
        println!("{:?}", book);
    }
    println!("Update volume by Book Title : ");
    let title = match Input(){
        Some(inputData) => inputData,
        None => return
    };
    let volume = match inputVolume(){
        Some(volume) => volume,
        None => return
    };
    if books.updateBook(&title, volume){
        println!("Book updated!")
    } else{
        println!("Book is not added yet!")
    }
}

fn Search_book(books: &mut HashInput){
        println!("Find a book by title : ");
        let title = match Input(){
            Some(inputData) => inputData,
            None => return
        };
        if books.searchBook(&title){
            println!("Book available!")
        } else {
            println!("Book unavailable, ask administrator!")
        }
}

fn lobby(){
    fn menu(){
        println!(" ");
        println!("### Library Admin System Information ###");
        println!("1. Add new Books");
        println!("2. Book list by System");
        println!("3. Update books");
        println!("4. Search books");
        println!("Select Menu : ")
    }
   
    loop{
        menu();
        {
            let choose = match Input(){
                Some(inputData) => inputData,
                None => return
            };

            let mut books = HashInput::new();

            match choose.as_str(){
                "1" => Add_new_book(&mut books),
                "2" => Book_list_books(&books),
                "3" => Update_books(&mut books),
                "4" => Search_book(&mut books),
                _=> break
            }
        }
    }
}

fn main(){
    lobby()
}