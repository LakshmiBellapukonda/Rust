fn main() {
    let mut vec: Vec<i32> = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    println!("Vector: {:?}", vec);

    let first_element = vec[0];
    let second_element = vec[1];
    println!("First element: {}", first_element);
    println!("Second element: {}", second_element);

    let removed_element = vec.pop();
    println!("Removed element: {:?}", removed_element);

    println!("Vector contents:");
    for value in &vec {
        println!("{}", value);

    }
    let mut vec_with_values = vec![10,20,30,40];
    println!("Vector with initial values: {:?}", vec_with_values);


    vec_with_values[2] = 35;
    println!("Modified vector: {:?}", vec_with_values);
    
    let length= vec_with_values.len();
    println!("Length of the vector: {}", length);
      

}
