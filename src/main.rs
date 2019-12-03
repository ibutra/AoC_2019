mod input;
mod one;
mod two;
mod three;

mod intcode;

use input::Input;

fn main() {
    let f = Input::new("input_1");
    one::run(f.as_i64());
    let f = Input::new("input_2");
    two::run(f.as_string());
    let f = Input::new("input_test");
    three::run(f.as_strings());
}
