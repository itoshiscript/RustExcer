use crate::utils::{Person, read_people, write_people};
mod utils;

fn main() {
    let person = Person::new();
    match write_people(vec![person]) {
        Ok(_) => println!("people.txt was written successfully"),
        Err(err) => println!("There was error while writing people.txt: {}", err),
    }

    match read_people() {
        Ok(_people) => println!("people.txt was read successfully"),
        Err(err) => println!("There was error while reading people.txt: {}", err),
    }
}
