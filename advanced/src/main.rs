use std::slice;
use std::ops::Add;

extern "C" {
    fn abs(input: i32) -> i32;
}
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
static mut COUNTER: u32 = 0;
//modifying static variable is unsafe
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
#[derive(Debug, PartialEq)]
struct Points{
    x:i32,
    y:i32
}
//operational overloading
impl Add for Points{
    type Output = Points;

    fn add(self, other_value:Points)->Points{
        Points{
            x:self.x+other_value.x,
            y:self.y+other_value.y,
        }
    }
}

//using a non default value for the RHS
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn main() {
    //raw pointers
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2 + 9);
    }

    unsafe fn stuff(x: i32) -> i32 {
        x
    };
    unsafe {
        println!("{}", stuff(45));
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    assert_eq!(
        Points { x: 1, y: 0 } + Points { x: 2, y: 3 },
        Points { x: 3, y: 3 }
    );

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    //type alias
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    //higher order func
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

//macro definition
#[macro_export]
macro_rules! vec {
    //match the rust code x for zero or more times
    ( $( $x:expr ),* ) => {
        {
            //instansiate a vector to be used in the macro
            let mut temp_vec = Vec::new();
            $(
                //push all the exprssion of x into the vector instance
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}