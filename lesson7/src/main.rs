pub mod file1;
pub use crate::file1::item;
pub mod file2;
pub use crate::file2::item2;
pub use crate::file2::item2::Summary;
use std::fmt::Display;
#[derive(Debug)]
struct Point<T>{
   x:T,
   y:T,
}
impl<T> Point<T> {
    fn return_x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct AnotherPoint<T, U>{
    x:T,
    y:U,
}
fn main() {
    let list=vec![1, 2, 45, 23, 122, 90, 9];
    let highest=item::get_highest(&list);
    println!("the highest number is {}", highest);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = item::largest_char(&char_list);
    println!("The largest char is {}", result);

    let float_list=vec![2.43, 5.90, 9.00, 19.70];
    let highest_float=item::largest(&float_list);
    println!("the largest float is {}", highest_float);

    println!("the longer word is : {}", item::longest_string("hello world", "hi"));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.67, y: 4.0 };
    println!("{:?}-{:?}", integer, float);
    println!("x integer {}, x float {}", integer.return_x(), float.return_x());
    println!("the distance from origin of float is {}", float.distance_from_origin());


    let mixed=AnotherPoint {x:30, y:19.23};
    println!("{} {}", mixed.x, mixed.y);

    let tweet= item2::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("author {}", tweet.summarize_author());
    item2::notify(tweet);

    fn _returns_summarizable() -> impl Summary {
        item2::Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
    //lifetime using sruct
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.part)
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}