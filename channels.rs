use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {received}");

    /*
    The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: 
    an Ok value holding a message if one is available and an Err value if there aren’t any messages this time.
     Using try_recv is useful if this thread has other work to do while waiting for messages: 
     we could write a loop that calls try_recv every so often, handles a message if one is available, 
     and otherwise does other work for a little while until checking again.

    We’ve used recv in this example for simplicity; we don’t have any other work for the main thread to do other than wait for messages,
    so blocking the main thread is appropriate.
    */

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // // In the main thread, we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator. For each value received, we’re printing it. When the channel is closed, iteration will end.

    // for received in rx {
    //     println!("Got: {received}");
    // }

    let (tx, rx) = mpsc::channel();

    // cloning transmitter to send from multiple threads
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}