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
    println!("Result is {}", add(4,6));

    // Expression - anything that returns a value
    // Staatement - anything that does not return a value - almost always ends with ;

    let weight: f64 = 80.5;
    let height: f64 = 189.5;

    let bmi: f64 = calculate_bmi(weight, height);

    println!("Your BMI: {:.2}", bmi);
}

fn test() {
    println!("Hello");
}

fn add(a: i32, b: i32) -> i32 {
    a + b // no return or semicolon- evaluates automaatically
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg * height_m
}
