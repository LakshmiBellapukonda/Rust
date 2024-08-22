use std::io;

fn main() {
    println!("Please enter a number:");
    let mut input = String::new();

    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    
    let doubled_number = number * 2;
    println!("The double of {} is {}", number, doubled_number);
}
