fn main() {
    let x = Some(5);
    let y = 6;

    match x {
        Some(z) => println!("Sum: {}", z+y),
        None => println!("nothing")
    }
}