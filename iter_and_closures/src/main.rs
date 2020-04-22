
use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,//closure
            value: None,
        }
    }
    //use to get value of the cache struct
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            //if the value exist, return value fron cache
            Some(v) => v,
            //if the value does not exist, add value to the calc closure and save it to value
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}


struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
//using iterator trait
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}



fn main() {
   let simulated_user_specified_value = 10;
   let simulated_random_number = 7;

   generate_workout(
       simulated_user_specified_value,
       simulated_random_number
   );
   
   let x = vec![1, 2, 3];
      //this takes ownership of the value of x
    let _equal_to_x = move |z:Vec<i32>| z == x;

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    let shoes=vec![
        Shoe{size:10, style:String::from("nike")},
        Shoe{size:12, style:String::from("adidas")}
    ];
    let my_size= shoes_in_my_size(shoes, 12);
    println!("{}", my_size[0].style);
    //sum of counter
    let sum_counter:u32=Counter::new().sum();
    println!("the sum of counter is {}", sum_counter);
}



fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_calc= Cacher::new(|num|{
        println!("calculating slowly!");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_calc.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_calc.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_calc.value(intensity)
            );
        }
    }
}