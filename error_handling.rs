fn main() {
    // Approach 1
    // enum Option<T> {
    //     Some(T), // represents a value
    //     None, // represents no value
    // }

    // enum Result<T, E> {
    //     Ok(T), // represents a value
    //     Err(E), // represents no value
    // }

    let result = divide(10.0, 0.0);

    match result {
        Some(x) => println!("Result of division is: {}", x),
        None => println!("Error"),
    }
}

fn divide(numberator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numberator / denominator)
    }
}

// fn divide(numberator: f64, denominator: f64) -> Result<f64, String> {
//     if denominator == 0.0 {
//         Err("Errpr".to_string())
//     } else {
//         Ok(numberator / denominator)
//     }
// }
