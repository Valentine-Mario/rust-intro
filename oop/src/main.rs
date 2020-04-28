mod lib;
pub use crate::lib::oop_exanple;
pub use crate::lib::simple_gui;
fn main() {
    let screen_item = simple_gui::Screen {
        components: vec![
            Box::new(simple_gui::Button {
                width: 34,
                height: 10,
                label: vec![String::from("submit"), String::from("retry")],
            }),
            Box::new(simple_gui::TextField {
                width: 90,
                height: 100,
                label: String::from("hello guys"),
            }),
            Box::new(simple_gui::Button {
                width: 44,
                height: 80,
                label: vec![String::from("submit"), String::from("retry")],
            }),
        ],
    };
    screen_item.run();
}
