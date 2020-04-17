fn main() {
    let x: i32=-5; //immutable variable
    println!("the value of x is {}", x);
    // //x=6;
    // println!("the value of x is {}", x);
    // let mut y=45; //mutable variable
    const MAX_POINTS: u32 = 100_000;
    println!("the constant value is {}", MAX_POINTS);

    let z=90;
    let z= z+100;
    println!("the value of z is {}", z);

    let name: String="hello people".to_string();
    let name=name.len();
    println!("{}",name);
    let flt_num=3.218;
    let flt_num2: f32= 3.123456789;
    println!("{} {}", flt_num, flt_num2);

    let _val:bool=true;
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z)=tup;
    println!("{}, {}, {}", x, y, z);
    println!("{}", tup.0);
    println!("{}",tup.1);
    println!("{}", tup.2);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    some_func("we".to_string());

    let x=90;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y and x is: {} {}", y, x);
    let summation=sum(23, 45);
    println!("{}", summation);
    let val=cond("hello world prople");
    println!("{}", val);

    println!("{}", cond2(false));

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter +1;
        }
    };
    println!("The result is {}", result);
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}


fn some_func(a:String){
   println!("{}", a);
}

fn sum(x:i32, y:i32)->i32{
    let value=x+y;
    value
}

fn cond(val:&str)->&str{
    let val_len=val.len();
    if val_len>10{
        "too long"
    }else if val_len<100{
        "too damn long"
    }else{
        "too short"
    }
}

fn cond2(val:bool)->i32{
    let number = if val {
        5
    } else {
        6
    };
    number
}