mod input;
// mod one;
// mod two;
// mod three;
// mod four;
mod five;

mod intcode;

use input::Input;

fn main() {
    // let f = Input::new("input_1");
    // one::run(f.as_i64());
    // let f = Input::new("input_2");
    // two::run(f.as_string());
    // let f = Input::new("input_3");
    // three::run(f.as_strings());
    // four::run(246515,739105);
    let f = Input::new("input_5");
    five::run(f.as_string());
}
