fn main() {
    // match x {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }

    //let x = 1;

    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    // let x = 1;

    // match x {
    //     1 | 2 => println!("one or two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    // let x = 'c';

    // match x {
    //     'a'..='j' => println!("early ASCII letter"),
    //     'k'..='z' => println!("late ASCII letter"),
    //     _ => println!("something else"),
    // }

    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    
    // fn main() {
    //     let p = Point { x: 0, y: 7 };
    
    //     let Point { x: a, y: b } = p;
    //     assert_eq!(0, a);
    //     assert_eq!(7, b);
    // }


    // We’ll receive an error because the s value will still be moved into _s, 
    // which prevents us from using s again. However, using the underscore by itself doesn’t ever bind to the value. 
    // this will compile without any errors because s doesn’t get moved into _:

    // let s = Some(String::from("Hello!"));

    // if let Some(_) = s {
    //     println!("found a string");
    // }

    // println!("{s:?}");

    // let num = Some(4);

    // match num {
    //     Some(x) if x % 2 == 0 => println!("The number {x} is even"),
    //     Some(x) => println!("The number {x} is odd"),
    //     None => (),
    // }


    // using if let instead 
    // let config_max = Some(3u8);
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {max}");
    // }
}