pub mod item {
    pub fn get_highest(a:&Vec<i32>)->i32{
        let mut largest=a[0];
        for item in a {
            if item > &largest {
                largest=*item;
            }
        }
        largest
    }
   pub fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
    
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }
    //generic implementation
    pub fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> T {
        let mut largest = list[0];
    
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    //specifying the parameter lifetime of the shorter param

    pub fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}