use crate::utils::{Person, read_people, write_people};

mod utils;

fn main() {
    match read_or_create_db() {
        Ok(people) => print_people(people),
        Err(err) => println!("There was error while reading people.txt: {}", err),
    }
}

fn print_people(people: Vec<Person>) {
    let kids: Vec<String> = people
        .into_iter()
        .filter(|person| person.age < 18)
        .filter(|person| person.age >= 10)
        .filter(|person| person.age < 80)
        .map(|person| format!("{} {}", person.first_name, person.last_name))
        .collect();

    let kid_names = kids.join(", ");

    println!(
        "people.txt was read successfully. These senior found: {}",
        kid_names
    );
}

fn create_db() -> std::io::Result<Vec<Person>> {
    let mut people = vec![];
    for _ in 0..4 {
        let person = Person::new();
        people.push(person)
    }

    write_people(people)?;
    read_people()
}

fn read_or_create_db() -> std::io::Result<Vec<Person>> {
    match read_people() {
        Ok(people) => {
            if people.len() < 4 {
                // not enough people in the database; let's re-create
                create_db()
            } else {
                Ok(people)
            }
        }
        Err(_err) => {
            // database error; let's re-create
            create_db()
        }
    }
}
