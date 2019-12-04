#[derive(Debug)]
struct Book{
    author: String,
    name: String
}

trait BookInformation {
    fn info(self) -> String;
}

impl BookInformation for Book{
    fn info(self) -> String{
        format!("{} are author of {}.",self.author,self.name)
    }
}

impl Book{

    fn new_book(author: String,name: String) -> Book{
        Book{
            author,
            name
        }
    }
}

fn main() {

    let book_01 = Book::new_book(String::from("Steve Klabnik and Carol Nichols"),String::from("[The Rust Programming Language]"));

    println!("{}",book_01.info());
}
