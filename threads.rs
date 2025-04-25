use std::thread;
use std::time::Duration;

fn main() {
    // Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.
    // so the spawned thread wont get to 10 here
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }
    
    // fix- waiting for thread execution with JoinHandle

    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }


    //wont work- cant use value from mian thread inside the spawned thread
    // handle.join().unwrap();

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {v:?}");
    // });

    // handle.join().unwrap();


    //By adding the move keyword before the closure, we force the closure to take ownership of the values
    //it’s using rather than allowing Rust to infer that it should borrow the value
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}