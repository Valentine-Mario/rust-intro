#[cfg(test)]
pub mod test_fun{
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    #[ignore]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    // #[test]
    // fn failed_test() {
    //     panic!("test fail");
    // }
    use super::some_mod;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = some_mod::Rect { width: 8, height: 7 };
        let smaller = some_mod::Rect { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
        assert!(smaller.canoot_hold(&larger));
    }
    use super::another_mod;
    #[test]
    fn test_three() {
       assert_eq!(5, another_mod::add_three(2));
       assert_ne!(6, another_mod::add_three(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = another_mod::greeting("Carol");
        assert!(result.contains("Carol"), "greeting func did not cpntain the name `{}`", result);
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        another_mod::Guess::new(200);
    }
}

mod some_mod{
    pub struct Rect{
        pub width:u32,
        pub height:u32,
    }

    impl Rect {
       pub fn can_hold(&self, other:&Rect)->bool{
           self.width>other.width && self.height>other.height
       }
       pub fn canoot_hold(&self, other:&Rect)->bool{
           self.width<other.width && self.height<other.height
       }
    }
}

pub mod another_mod{
    pub fn add_three(a:i32)->i32{
        a+3
    }
    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    pub struct Guess {
        value: i32,
    }
    
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
    
            Guess {
                value
            }
        }
    }

}