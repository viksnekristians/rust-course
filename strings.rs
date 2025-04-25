fn main() {
    let sl = "This is string literal";

    println!("String literal is {}", sl);

    let s1: String = String::from("Hello");
    let s2 = String::from("world");
    // s3 took ownership of s1- s1 no longer exists
    let s3 = s1 + &s2;
}