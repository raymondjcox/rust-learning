trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn notify<T: Summary>(item: T) {
    println!("Notify! {}", item.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("wat"),
        location: String::from("Cambridge"),
        author: String::from("Raymond Cox"),
        content: String::from("Some content")
    };
    notify(article);
}
