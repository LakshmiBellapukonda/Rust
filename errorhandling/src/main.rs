fn main() {
    let input = "42";  
    let invalid_input = "abc";  


    match parse_integer(input) {
        Ok(value) => println!("Parsed integer: {}", value),
        Err(e) => eprintln!("Error: {}", e),
    }

    
    match parse_integer(invalid_input) {
        Ok(value) => println!("Parsed integer: {}", value),
        Err(e) => eprintln!("Error: {}", e),
    }
}


fn parse_integer(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Failed to parse integer".to_string()),
    }
}
