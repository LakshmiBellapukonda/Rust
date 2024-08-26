fn main() {
    let mut my_string = String::from("Hello, Rust!");

    println!("Original string: {}", my_string);

    let length = get_length(&my_string); 
    println!("The length of the string is: {}", length);

    append_exclamation(&mut my_string); 
    println!("Modified string: {}", my_string);
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn append_exclamation(s: &mut String) {
    s.push_str("!"); 
}
