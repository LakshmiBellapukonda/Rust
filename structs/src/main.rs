 struct Person {
    name : String,
    age : u32,
 }
 
 
 fn main() {
    let person1 = Person {
        name : String::from("raha"),
        age : 10,
    };

    let person2 = Person {
        name : String::from("vayu"),
        age : 15,
    };

    println!("{} is {} yeras old", person1.name, person1.age);

    println!("{} is {} years old", person2.name, person2.age);
}
