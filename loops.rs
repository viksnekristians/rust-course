// loop
// while
// for

fn main() {
    // loop {
    //     println!("Hello"); // infinitely prints this
    // }

    let mut counter = 0;
    // good use example- retry an operation that might fail
    let result = loop {
        counter += 1;

        if counter > 10 {
            break counter - 2
        }
    };

    println!("Result is {}", result);

    // Loop labels- var arī loop u.c.
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            println!("i: {}, j: {}", i, j);
            if i == 2 && j == 2 {
                println!("Breaking out of the outer loop");
                break 'outer; // Break out of the outer loop
            }
        }
    }
    println!("Exited the loops");

    // while loop
    let mut number = 0;
    while number < 10 {
        println!("{number}");
        number += 1;
    }

    // for in
    // for element in arr {...}
}