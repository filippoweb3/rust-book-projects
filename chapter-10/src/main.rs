struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> { // impl over generic type
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2> (self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

use core::prelude;
use std::fmt::Debug;

use aggregator::{notify, NewsArticle, Summary, Tweet, Pair};

// impl Point<f32> implementation over concrete type

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    
    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    let result = largest_i32(&number_list);

    println!("The largest number is {result}.");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);

    println!("The largest number is {result}.");

    let result = largest(&char_list);

    println!("The largest number is {result}.");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0};

    println!("integer.x = {}", integer.x());

    let p1 = Point2 { x: 5, y: 10.4};
    let p2 = Point2 { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from(
            "Penguins win the Stanley Cup Championship!"
        ),
        location: String::from("Pittsburg, PA, USA"),
        author: String::from("Iceburg"),
        content: String::from(
            "The Pittsburg Penguins.",
        ),
    };

    println!("New article available! {}", article.summarize());

    println!("{:?}", notify(&tweet));

    dbg!(println!("{:?}", returns_summarizable()));

    let pair_of_integers = Pair::new("true", "false");

    println!("{:?}", pair_of_integers.cmp_display());

    println!("{:?}", Pair::new("true", "false"));

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);
    println!("The longest string is {result}.");

}

fn largest_i32 (list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest

}

fn largest_char (list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest

}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest

}

fn returns_summarizable() -> impl Summary + Debug {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
