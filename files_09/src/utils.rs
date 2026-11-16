use std::fs::{read_to_string, write};
use std::str::FromStr;
use std::str::Split;

pub struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    // pub fn print(&self) {
    //     println!("Hello {} {} {}", self.first_name, self.last_name, self.age);
    // }

    pub fn new() -> Self {
        let first_name = ask_for_a_string("What is your first name?");
        let last_name = ask_for_a_string("What is your last name?");
        let age = ask_for_a_number("How old are you?").unwrap_or(0);
        Person {
            first_name,
            last_name,
            age,
        }
    }
}

pub fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input.trim().to_string()
}

pub fn read_number() -> Result<u8, String> {
    let input = read_string();

    if input.is_empty() {
        Err("You did not enter any data".to_string())
    } else {
        u8::from_str(&input).or(Err("You've entered an invalid number".to_string()))
    }
}

pub fn ask_for_a_string(question: &str) -> String {
    println!("{}", question);
    read_string()
}

pub fn ask_for_a_number(question: &str) -> Result<u8, String> {
    println!("{}", question);
    read_number()
}

pub fn write_people(people: Vec<Person>) -> std::io::Result<()> {
    let mut output = String::new();

    for person in people {
        output.push_str(&person.first_name);
        output.push('\n');
        output.push_str(&person.last_name);
        output.push('\n');
        output.push_str(&person.age.to_string());
        output.push('\n');
    }

    write("people.txt", output)
}

pub fn read_person(lines: &mut Split<'_, char>) -> Option<Person> {
    let first_name = lines.next()?;
    let last_name = lines.next()?;
    let age_as_string = lines.next()?;
    let age = u8::from_str(age_as_string).unwrap_or(0);
    let person = Person {
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        age,
    };
    Some(person)
}

pub fn read_people() -> Result<Vec<Person>, std::io::Error> {
    let input = read_to_string("people.txt")?;
    let mut lines = input.split('\n');
    let mut people = vec![];
    while let Some(person) = read_person(&mut lines) {
        people.push(person)
    }
    Ok(people)
}
