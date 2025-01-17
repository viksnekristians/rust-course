// Compound types
fn main() {
    //Arrays- all elements need to be same type, fixed size
    let numbers: [i32; 5] = [1,2,3,4,5]; // type, length  
    println!("Number array: {:?}", numbers);
    let fruits: [&str; 3] = ["Apple", "banana", "kiwi"];
    println!("Fruits array: {:?}", fruits);

    //Tuples- can have different types and has fixed size
    let person: (String, i32, bool) = ("Alice".to_string(), 30, false); // types not mandatory
    println!("Tuple: {:?}", person);

    //Slices: elements are in sequence in memory which is good for memory use
    //Slices allow you to work with a contiguous block of data (like an array) without copying the data.

    let number_slices: &[i32] = &[1,2,3,4,5];  
    println!("Number slice: {:?}", number_slices);

    let animal_slices :&[&str] = &["Lion", "Crocodile"];  
    println!("Animal slice: {:?}", animal_slices);

    let book_slices :&[&String] = &[&"Harry Potter".to_string(), &"Crocodile Dandy".to_string()];  
    println!("Book slice: {:?}", book_slices);

    /*
    &[&str]: This means `animal_slices` is a reference to a slice of string slices. 
    A slice in Rust is a dynamically-sized view into a contiguous block of memory, 
    often used with arrays. `&str` represents a borrowed string slice.
    
    &["Lion", "Crocodile"]:
    Here, the `&` takes a reference to the array `["Lion", "Crocodile"]`, creating a slice.
    The type of the array literal is `[&str; 2]` (an array of two string slices), and by using `&`, it becomes a slice (`&[&str]`), which is dynamically sized.
    */
}