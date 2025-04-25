use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {m:?}");

    // let counter = Mutex::new(0);
    // let mut handles = vec![];

    // let counter = Mutex::new(0);
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());
    //wont work


    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;

            println!("adding");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

//     Mutex<T> ensures safe access to shared data.
//     Arc<T> enables multiple threads to share ownership of the same data.
//     lock().unwrap() grants exclusive access to the mutex-protected value.
//     Thread safety is achieved by combining Arc (ownership) and Mutex (synchronization).
//     Calling .join() ensures all threads complete before proceeding.
}
