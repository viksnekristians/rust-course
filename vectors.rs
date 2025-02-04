/*
 vectors allow to store more than single value of the same type
 in a single structure. All values will be next to each other in memory
 */
fn main() {
    let mut _vec: Vec<i32> = Vec::new();
    let _other_vec: Vec<i32> = vec![1,2,3];

    // like dynamic array that can grow or shrink as needed

    _vec.push(1);
    _vec.push(2);
    _vec.push(3);

    println!("{:?}", _vec);

    let third: &i32 = &_vec[2]; // direct indexing- reference to 3rd value

    let third = _vec.get(2);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("Index out of bounds"),
    };
}