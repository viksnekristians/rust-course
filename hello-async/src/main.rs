// use trpl::{Either, Html};
// use std::time::Duration;

// // async fn page_title(url: &str) -> (&str, Option<String>) {
// //     let text = trpl::get(url).await.text().await;
// //     let title = Html::parse(&text)
// //         .select_first("title")
// //         .map(|title| title.inner_html());
// //     (url, title)
// // }

// // fn main() {
// //     // let args: Vec<String> = std::env::args().collect();

// //     // trpl::run(async {
// //     //     let url = &args[1];
// //     //     match page_title(url).await {
// //     //         Some(title) => println!("The title for {url} was {title}"),
// //     //         None => println!("{url} had no title"),
// //     //     }
// //     // })

// //     let args: Vec<String> = std::env::args().collect();

// //     trpl::run(async {
// //         let title_fut_1 = page_title(&args[1]);
// //         let title_fut_2 = page_title(&args[2]);

// //         let (url, maybe_title) =
// //             match trpl::race(title_fut_1, title_fut_2).await {
// //                 Either::Left(left) => left,
// //                 Either::Right(right) => right,
// //             };

// //         println!("{url} returned first");
// //         match maybe_title {
// //             Some(title) => println!("Its page title is: '{title}'"),
// //             None => println!("Its title could not be parsed."),
// //         }
// //     })
// // }

// fn main() {
//     // this example will not wait until saawned task finishes- will shut down spawned task
//     // as soon as loop in main function finishes:
//     // trpl::run(async {
//     //     trpl::spawn_task(async {
//     //         for i in 1..10 {
//     //             println!("hi number {i} from the first task!");
//     //             trpl::sleep(Duration::from_millis(500)).await;
//     //         }
//     //     });

//     //     for i in 1..5 {
//     //         println!("hi number {i} from the second task!");
//     //         trpl::sleep(Duration::from_millis(500)).await;
//     //     }
//     // });

//     // this solves it:
//     // let handle = trpl::spawn_task(async {
//     //     for i in 1..10 {
//     //         println!("hi number {i} from the first task!");
//     //         trpl::sleep(Duration::from_millis(500)).await;
//     //     }
//     // });

//     // for i in 1..5 {
//     //     println!("hi number {i} from the second task!");
//     //     trpl::sleep(Duration::from_millis(500)).await;
//     // }

//     // handle.await.unwrap();

//     // The bigger difference is that we didn’t need to spawn another operating system thread to do this. 
//     // In fact, we don’t even need to spawn a task here. Because async blocks compile to anonymous futures, 
//     // we can put each loop in an async block and have the runtime run them both to completion using the trpl::join function.



//     trpl::run(async {
        
//         // let fut1 = async {
//         //     for i in 1..10 {
//         //         println!("hi number {i} from the first task!");
//         //         trpl::sleep(Duration::from_millis(500)).await;
//         //     }
//         // };

//         // let fut2 = async {
//         //     for i in 1..5 {
//         //         println!("hi number {i} from the second task!");
//         //         trpl::sleep(Duration::from_millis(500)).await;
//         //     }
//         // };

//         // trpl::join(fut1, fut2).await;

//         let (tx, mut rx) = trpl::channel();

//         let tx_fut = async {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];

//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let rx_fut = async {
//             while let Some(value) = rx.recv().await {
//                 println!("received '{value}'");
//             }
//         };

//         println!("hello");

//         trpl::join(tx_fut, rx_fut).await;

//     });


//     // using channels for concurrency:
//     // let (tx, mut rx) = trpl::channel();

//     // let val = String::from("hi");
//     // tx.send(val).unwrap();

//     // let received = rx.recv().await.unwrap();
//     // println!("Got: {received}");
// }

// use trpl::Either;
// use std::time::Duration;
// use std::future::Future;

// // --snip--

// fn main() {
//     trpl::run(async {
//         let slow = async {
//             trpl::sleep(Duration::from_secs(5)).await;
//             "Finally finished"
//         };

//         match try_long_task(slow, Duration::from_secs(2)).await {
//             Ok(message) => println!("Succeeded with '{message}'"),
//             Err(duration) => {
//                 println!("Failed after {} seconds", duration.as_secs())
//             }
//         }
//     });
// }

// async fn try_long_task<F: Future>(
//     future_to_try: F,
//     max_time: Duration,
// ) -> Result<F::Output, Duration> {
//     match trpl::race(future_to_try, trpl::sleep(max_time)).await {
//         Either::Left(output) => Ok(output),
//         Either::Right(_) => Err(max_time),
//     }
// }

use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    })
}

//stream of messages as a stand-in for a stream of data we might see from a WebSocket or another real-time communication protocol
fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
