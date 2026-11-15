fn main() {
    println!("What is your name?");
    let input = read_clean_string();
    println!("Your name is: {}", input);
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input
}

fn read_clean_string() -> String {
    let new_input = read_string();
    new_input.trim().to_string()
}
