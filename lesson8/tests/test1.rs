#[path = "../src/run_test.rs"]
pub mod app;
pub use self::app::another_mod;

pub mod common;
pub use crate::common::some_block;
#[test]
fn it_adds_two() {
    some_block::stuff();
    assert_eq!(5, another_mod::add_three(2));
}