fn display_tasks<'a>(tasks: &'a [&'a str]) {
    for task in tasks {
        println!("- {}", task);
    }
}

fn add_task<'a>(tasks: &mut Vec<&'a str>, new_task: &'a str) {
    tasks.push(new_task);
}

fn main() {
    let mut tasks = vec!["Buy groceries", "Clean the house"];
    println!("Initial tasks:");
    display_tasks(&tasks);

    add_task(&mut tasks, "Read a book");

    println!("\nUpdated tasks:");
    display_tasks(&tasks);
}
