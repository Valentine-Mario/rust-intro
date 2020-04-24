use add_one;
use add_two;
fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
    println!("add 3 with 2 results in {}", add_two::add_two(3))
}