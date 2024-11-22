pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!(
            "(Read more from {}...)",
            self.summarize_author()
        )
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

//impl Summary for NewsArticle {
//    fn summarize(&self) -> String {
//        format!(
//            "{}, by {} ({})",
//            self.headline,
//            self.author,
//            self.location
//        )
//    }
//}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

//trait bound
//pub fn notify<T: Summary>(item: &T) {}
//pub fn notify<T: Summary>(item1: &T, item2: &T) {} // good when we have 2 param

#[derive(Debug)]
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}
impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: core::fmt::Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is {}.", self.x);
        } else {
            println!("The largest number is {}.", self.y)
        }
    }
} 