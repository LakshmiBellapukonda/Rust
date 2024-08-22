use std::io::{self, Write};

fn main() {
    let mut name = String::new();
    print!("Enter your name: ");
    io::stdout().flush().expect("Failed to flush stdout"); 
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); 

    
    let mut age_str = String::new();
    print!("Enter your age: ");
    io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt is printed before user input
    io::stdin().read_line(&mut age_str).expect("Failed to read line");
    let age: u32 = match age_str.trim().parse() {
        Ok(age) => age,
        Err(_) => {
            eprintln!("Invalid age input. Please enter a number.");
            return;
        }
    };

    
    println!("Hello, {}!", name);

    if age >= 18 {
        println!("You are old enough to vote.");
    } else {
        println!("You are not old enough to vote.");
    }
}

