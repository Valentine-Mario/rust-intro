use std::env;
use std::process;
mod lib;
pub use crate::lib::arg_config;
pub use crate::lib::run;

fn main() {
    //take a collection of argumants and read them
    let args: Vec<String> = env::args().collect();
    let config_value =  arg_config::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run::read_file(&config_value){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

