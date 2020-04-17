#[derive(Debug)]
struct User {
    name:String,
    email:String,
    age:u32,
    is_admin:bool,
}
struct Rectangle{
    width:u32,
    height:u32,
}
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
struct Triangle{
    base:f32,
    height:f32
}

impl Triangle{
    fn area(&self)->f32{
        0.5*self.base*self.height
    }
    fn can_hold(&self, other:&Triangle)->bool{
        self.base > other.base && self.height>other.height
    }
}

enum IpaddrKind {
   V4(u8, u8, u8, u8), 
   V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
    }
}

enum Coin{
    Penny,
    Nickle, 
    Cent,
}

fn main() {

    let mut ada=User{
        name:String::from("Ada"),
        email:String::from("ada@gmail.com"),
        age:20,
        is_admin:false,
    };
    let charles=User{
        name:String::from("Charles"),
        email:String::from("charles@gmail.com"),
        //copy the remaining value from ada struct
        ..ada
    };

    ada.name=String::from("Adaobi");

    println!("{:#?}, {:#?}", ada, charles);
    //tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let rec1=(3, 4);
    println!("the area of this rect is {}", area_1(rec1));

    let rec2=Rectangle{width:30, height:40};
    println!("the area of this rect is {}", area_2(&rec2));

    let tri1=Triangle{base:20.0, height:30.0};
    let tri2=Triangle{base:10.0, height:23.90};
    println!("the area of the triangle is {}", tri1.area());
    println!("tri1 can hold tri2: {}", tri1.can_hold(&tri2));
    //calling an associative method
    let _sq=Rectangle::square(3);
   // println!("{}", sq.width)

   let four=IpaddrKind::V4(12, 1, 0, 120);
   let six=IpaddrKind::V6(String::from("::1"));
   let msg1=Message::Move {x:23, y:34,};
   msg1.call();
   let val1= Some("this is a value");
   let none_val:Option<i32>=Some(90);
   println!("{:?}", none_val);

   let cent=Coin::Cent;
   println!("{}",check_coin(&cent));
   let a:Option<i32>=Some(90);
   println!("{:?}", plus_one(a));

   let some_u8_value = Some(0u8);
   //an alternate to match with less code
   if let Some(3) = some_u8_value {
    println!("three");
}else{
    //equivalent to _=>value
    println!("not three");
}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn check_coin(coin:&Coin)->u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Cent => 10,
        _=>100,
    }
}

//area using tuple
fn area_1(dim:(u32, u32))->u32{
    dim.0*dim.1
}

fn area_2(rect:&Rectangle)->u32{
    rect.height*rect.width
}