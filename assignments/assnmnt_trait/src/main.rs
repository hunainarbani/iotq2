//Summary Signature
pub trait Summary {
    fn summarize(&self) -> String;            
}
//End Summary Signature

//Custom Types
#[derive(Debug)]
pub struct NewsArticle {
    author: String,
    content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String{
        format!("{}\nSubmitted by {}", self.content, self.author)
    }
}
#[derive(Debug)]
pub struct Tweet {
    username: String,
    content: String
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Checking Assignment Upload Status.\n{}: {}", self.username, self.content)
    }
}
//End Custom Types
fn main() {
    
    let nart = NewsArticle {
        author: String::from("Hunain Arbani"),
        content: String::from("This is First Assignment for IOT-Q2")
    };

    println!("{}",nart.summarize());

    let newtweet = Tweet{
        username: String::from("harbani"),
        content: String::from("Assignemnt has been Uploaded.")
    };

    println!("{}",newtweet.summarize());

}
