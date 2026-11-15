use crate::utils::{ask_for_a_number, ask_for_a_string};
use std::fs::write;

mod utils;

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
    let first_name = ask_for_a_string("What is your first name?");
    let last_name = ask_for_a_string("What is your last name?");
    let age = ask_for_a_number("How old are you?").unwrap_or(0);
    let person = Person {
        first_name,
        last_name,
        age,
    };

    match write_person(&person) {
        Ok(_) => println!("Successfully created file"),
        Err(err) => println!("Failed to create file: {}", err),
    };
}

fn write_person(person: &Person) -> std::io::Result<()> {
    let mut output = String::new();
    output.push_str(&person.first_name);
    output.push('\n');
    output.push_str(&person.last_name);
    output.push('\n');
    output.push_str(&person.age.to_string());
    output.push('\n');
    write("person.txt", output)
}
