use std::str::FromStr;

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
