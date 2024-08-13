enum Animal{
    Dog(String),
    Cat(String),
    Bird(String),
}



fn main() {
    let my_dog = Animal::Dog(String::from("Rex"));
    let my_cat = Animal::Cat(String::from("Edward"));
    let my_bird = Animal::Bird(String::from("Sweety"));

    describe_animal(&my_dog);
    describe_animal(&my_cat);
    describe_animal(&my_bird);


}
fn describe_animal (animal: &Animal) {
    match animal{
        Animal::Dog(name) => println!("This is dog named {}", name),
        Animal::Cat(name) => println!("This is cat named {}", name),
        Animal::Bird(name) => println!("This is bird named {}", name),
    }
}
