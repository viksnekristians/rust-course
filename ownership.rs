// ownership, borrowing, references

// ownership

// for solving performance issues relateed to garbage collector
// RUST- memory safe
// every value has a single owner.
// every variable has one value and it is its sole owner

// 3 rules
/*
- every value has an owner
- there can only be one owner at time
- when owner goes out of scope. the value will be dropped
*/


fn main() {
    let s1 = String::from("RUST"); // s1- owner of the string
    let len = calculate_length(&s1); // borrowing and passing by reference

    println!("Length is: {}", len);

    let s2 = s1; // ownership of string is transferred from s1 to s2

    //println!("{}", s1); would fail because s1 doesnt own the string anymore
    println!("{}", s2);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}