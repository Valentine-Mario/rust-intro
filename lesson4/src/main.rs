use std::collections::HashMap;
use std::collections::*;
use std::{cmp::Ordering, io};
mod testing_stuff;
pub use crate::testing_stuff::stuff;
#[path = "./rest/rest.rs"] // Here
pub mod rest;
use self::rest::rester;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    stuff::test();
    println!("{}", rester::sum(20.90, 34.98));
}
