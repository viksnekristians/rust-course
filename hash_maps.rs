use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 7);
    scores.insert(String::from("Black"), 6);

    let team_name: String = String::from("Black");

    let score: &i32 = &scores.get(&team_name).copied().unwrap_or(0);

    println!("Black score is {}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}