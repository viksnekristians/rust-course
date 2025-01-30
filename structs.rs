fn main() {
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User {
        active: bool,
        username: String,
        sign_in_count: u64,
    }

    let mut user1: User = User{
        active: true,
        username: String::from("test"),
        sign_in_count: 5,
    };

    user1.username = String::from("test2");

    println!("username is {}", user1.username);

    // from function
    let user2 = build_user(String::from("aaa"));
    println!("username is {}", user2.username);

    // from other instance
    let user3: User = User {
        username: String::from("newtest"),
        ..user2
    };

    println!("username is {}", user3.username);

    fn build_user(username: String) -> User {
        User {
            active: true,
            username: username,
            sign_in_count: 5,
        }
    }

    // Tuple struct
    struct Color(i32, i32, i32);

    let black: Color = Color(0, 0, 0);

    // unit-like struct
    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;
}

