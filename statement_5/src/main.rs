fn main() {
    let mut count =0;

    loop {
        count += 1;
        println!("Count is: {}", count);

        if count >= 5 {
            break;
        }
    }
     println!("Loop has ended: {}", count);
}
