fn main() {
   let mut str=String::from("hello");
   //mutating a string
   str.push_str(" world", );
   println!("{}", str);

   let str2= String::from("this is a string");
   //use clone for deep copy
   let str3=str2.clone();
   println!("{} {}", str2, str3);

   let s=String::from("this");
   take_ownership(s); 
   // s is no longer valid due to ownership rule

   let a=7;
   make_copy(a);
   //a can still be accessed
    println!("{}", a);

    //get the length of a string without taking ownership of it's value using refrencing
    let s2=String::from("value one");
    let s2_len=calc_len(&s2);
    println!("{} {}", s2_len, s2);

    //mutating a referene value
    let mut s3=String::from("some other value");
    change_val(&mut s3);

    let mut s4 = String::from("hello");

    let r1 = &s4; // no problem
    let r2 = &s4; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s4; // no problem
    println!("{}", r3);

    
    //let reference_to_nothing = dangle();
    let s5=String::from("Over to you");

    let word1=&s5[0..4];
    let word2=&s5[4..7];

    println!("{} {}", word1, word2);

    //return the first word
    println!("{}", first_word(&String::from("this is a string")));
}


fn take_ownership(s:String){
    println!("{}", s)
}
fn make_copy(a:i32){
    println!("{}", a)
}

fn calc_len(s: &String)->usize{
    s.len()
}

fn change_val(s: &mut String){
    s.push_str(" has been added");
    println!("{}",s)
}
//try creating a dangling refernce
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

