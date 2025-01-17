fn main() {
    // Functions

    test();

    // block
    // The last line, price * qty, is an expression without a semicolon, which means the block evaluates to this value.
    let _x = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // `_x` will be of type `i32`
    };

    println!("Result is {}", _x);

    println!("Result is {}", add(5,5));
}

fn test() {
    println!("Hello");
}

fn add(a: i32, b: i32) -> i32 {
    a + b // no return or semicolon- evaluates automaatically
}
