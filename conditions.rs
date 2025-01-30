fn main() {
    // if expressions
    let age = 18;
    if age >= 18 {
        println!("You are old enough to vote");
    } else {
        println!("You are not old enough to vote!");
    }

    let number: u16 = 6;
    if number % 4 == 0 {
        println!("number is divisable by 4");
    } else if number % 3 == 0 {
        println!("number is divisable by 3");
    } else {
        println!("number is divisable by none of these");
    }

    let condition = true;
    let number = if condition {5} else {6};
}