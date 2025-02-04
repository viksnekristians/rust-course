fn main() {
    let s1: String = String::from("Hello");
    let s2 = String::from("world");
    // s3 took ownership of s1- s1 no longer exists
    let s3 = s1 + &s2;
}