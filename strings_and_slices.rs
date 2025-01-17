fn main() {
    // Strings
    /*
    - owns the string data
    - mutable if declared as mut
    - stored in heap memory
    */
    let mut s: String = String::from("hello");
    s.push_str(" world");

    println!("String: {}", s);

    // String slices
    /*
    - does not own- it borrows from String or string literal
    - immutable
    - may point to heap or binary
    - fixed and read only
    */

    let string: String = String::from("Hello, world");  
    let slice: &str = &string[0..5]; // references memory where string is. points to 0 byte and length is 5

    println!("{}", slice);
}