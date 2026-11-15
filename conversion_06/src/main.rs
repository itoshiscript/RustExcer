use std::str::FromStr;

fn main() {
    println!("How old are you?");
    match read_number() {
        None => println!("You've entered an invalid age"),
        Some(age) => println!("You are {} years old.", age),
    }
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input.trim().to_string()
}

fn read_number() -> Option<u8> {
    let input = read_string();
    u8::from_str(&input).ok()
}
