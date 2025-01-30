// shadowing is not the same as marking variable as mutable

fn main() {
    let x = 5; // old x
    let x = x + 1; // new x. old is shadowed by this variable 6
    let x = x + 1; // 7
    // ...

    {
        let x = x * 2;
        println!("Value in inner scope: {}", x); // 14
    }
    println!("Value: {}", x); // 7

    // create variable with same nme and overshadow the first one
}