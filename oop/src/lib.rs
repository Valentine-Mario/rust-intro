pub mod oop_exanple {
    pub struct AverageCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AverageCollection {
        //function to compute average
        fn compute_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }

        //function to return the average
        pub fn average(&self) -> f64 {
            self.average
        }

        //function to add item to list
        pub fn add_item(&mut self, item: i32) {
            self.list.push(item);
            self.compute_average();
        }

        //function to remove item from list
        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.compute_average();
                    Some(value)
                }
                None => None,
            }
        }
    }
}

pub mod simple_gui {
    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: Vec<String>,
    }

    impl Draw for Button {
        fn draw(&self) {
            // code to actually draw a button
        }
    }
    pub struct TextField {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Draw for TextField {
        fn draw(&self) {
            //do stuff
        }
    }
}
