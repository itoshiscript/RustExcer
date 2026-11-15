use crate::utils::{ask_for_a_number, ask_for_a_string};
mod utils;

fn main() {
    ask_for_a_string("What is your name?");

    match ask_for_a_number("How old are you?") {
        Ok(age) => println!("You are {} years old.", age),
        Err(err) => println!("{}", err),
    }
}
