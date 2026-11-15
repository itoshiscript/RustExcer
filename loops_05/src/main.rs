fn main() {
    let list_of_numbers = vec![10, 25, 8, 14, 3, 42];
    println!("I can list these numbers:");
    for number in &list_of_numbers {
        print_number(number)
    }
}

fn print_number(number: &i32) {
    print!("{number},")
}
