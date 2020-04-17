
pub mod vector_eg {
   pub fn vec_fun(a:i32)->Vec<i32>{
        let mut v: Vec<i32> = Vec::new();
        v.push(a);
        v
    }
    pub mod spredsheet{
        pub enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
    }
    pub fn enum_vec(num1:i32, name:String, num2:f64)->Vec<spredsheet::SpreadsheetCell>{
        
        let row = vec![
            spredsheet::SpreadsheetCell::Int(num1),
            spredsheet::SpreadsheetCell::Text(name),
            spredsheet::SpreadsheetCell::Float(num2),
        ];
        row
    }
}