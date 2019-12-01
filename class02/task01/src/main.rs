#[derive(Debug)]

struct Book {
    author :String,
    title: String
}

impl Book {

    fn getBook(author: String,title: String) -> Book{

        Book {
            author,
            title
        }
    }

    fn BookInfo(&self)-> String{

        format!("{} written by {}",self.title,self.author)
    }
}

fn main() {
    
    let book_01 = Book{author: "Hunain Arbani".to_string(),
                       title: "The Kitaab".to_string()
    };


    println!("{}",book_01.BookInfo());


}
