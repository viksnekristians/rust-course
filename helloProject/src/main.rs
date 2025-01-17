fn main() {
    // primitive variable types
    let string: String = String::from("Hello, world");
    let slice: &str = &string[0..5];

    println!("{}", slice);

    test();

    let _x: () = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // bez return. semicolon- evaluates automatically
    };

    println!("Result is {}", _x);
}

fn test() {
    println!("Hello");
}

fn add(a: i32, b: i32) -> i32 {
    a + b // no return or semicolon- evaluates automaatically
}

const _X = {};