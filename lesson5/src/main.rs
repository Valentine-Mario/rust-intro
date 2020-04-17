#[path = "./app/using_vec.rs"]
pub mod app;
pub use self::app::vector_eg;
#[path = "./app/another_fun.rs"]
pub mod app2;
pub use self::app2::delit_mod;
use std::collections::HashMap;

fn main() {
    let vec1 = vector_eg::vec_fun(20);
    println!("{:?}", vec1);
    delit_mod::delit("some name");
    let mut v = vec![1, 2, 3];
    let first = v[0];
    v.push(6);
    println!("The first element is: {}", first);

    for i in &mut v {
        *i += 10; //derefrence value before using them
    }
    let sec: &i32 = &v[1];
    println!("the second element is {}", sec);

    match v.get(100) {
        Some(sec) => println!("the second element is {}", sec),
        None => println!("no 100th element there are only {} in this vector", v.len()),
    }
    println!("{:?}", v);
    let enum_val: Vec<vector_eg::spredsheet::SpreadsheetCell> =
        vector_eg::enum_vec(23, String::from("hello"), 30.6);
    match enum_val[0] {
        vector_eg::spredsheet::SpreadsheetCell::Int(value) => println!("{}", value),
        _ => println!("stuff"),
    }
    for i in "hello".chars() {
        println!("{}", i);
    }
    for i in "hello".bytes() {
        println!("{}", i);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10.0);
    scores.insert(String::from("Yellow"), 50.0);
    println!("{:?}", scores);
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
