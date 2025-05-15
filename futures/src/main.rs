use trpl::{Either, Html};
//use std::time::Duration;
use std::{thread};
async fn page_title(url: &str)-> (&str, Option<String>){
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

// fn main() {
//
//     let args: Vec<String> = std::env::args().collect();
//
//     // trpl::run(async {
//     //     let url = &args[1];
//     //     match page_title(url).await {
//     //         Some(title) => println!("The title for {url} was {title}"),
//     //         None => println!("{url} had no title"),
//     //     }
//     // });
//     trpl::run(async {
//         let title_fut_1 = page_title(&args[1]);
//         let title_fut_2 = page_title(&args[2]);
//
//         let (url, maybe_title) =
//             match trpl::race(title_fut_1, title_fut_2).await {
//                 Either::Left(left) => left,
//                 Either::Right(right) => right,
//             };
//
//         println!("{url} returned first");
//         match maybe_title {
//             Some(title) => println!("Its page title is: '{title}'"),
//             None => println!("Its title could not be parsed."),
//         }
//     });
//     trpl::run(async {
//         let handle = trpl::spawn_task(async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         });
//
//         for i in 1..5 {
//             println!("hi number {i} from the second task!");
//             trpl::sleep(Duration::from_millis(500)).await;
//         }
//
//         handle.await.unwrap();
//     });
//     trpl::run(async {
//         let fut1 = async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };
//
//         let fut2 = async {
//             for i in 1..5 {
//                 println!("hi number {i} from the second task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };
//         trpl::join(fut1, fut2).await;
//     });
//
//     trpl::run(async {
//         //async_test1().await;
//         //async_test2().await;
//
//         trpl::join(async_test1(), async_test2()).await;
//     });
//
//     for i in 1..5 {
//         println!("hi number {i} from the third task!");
//     }
//
//     trpl::run(async {
//         let (tx, mut rx) = trpl::channel();
//
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("future"),
//         ];
//
//         for val in vals {
//             println!("sent '{val}'");
//             tx.send(val).unwrap();
//             trpl::sleep(Duration::from_millis(500)).await;
//         }
//
//         while let Some(value) = rx.recv().await {
//             println!("received '{value}'");
//         }
//     })
//
// }
//
// async fn async_test1() {
//     for i in 1..5 {
//         println!("hi number {i} from the first task!");
//         trpl::sleep(Duration::from_millis(1000)).await;
//     }
// }
//
// async fn async_test2() {
//     for i in 1..10 {
//         println!("hi number {i} from the second task!");
//         trpl::sleep(Duration::from_millis(500)).await;
//     }
// }

// fn main() {
//     trpl::run(async {
//         let (tx, mut rx) = trpl::channel();
//         let tx_fut = async move {
//             let vals = vec![
//                 String::from("hi"),
//                 String::from("from"),
//                 String::from("the"),
//                 String::from("future"),
//             ];
//
//             for val in vals {
//                 tx.send(val).unwrap();
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };
//
//         let rx_fut = async {
//             while let Some(value) = rx.recv().await {
//                 println!("received '{value}'");
//             }
//         };
//
//         trpl::join(tx_fut, rx_fut).await;
//
//     })
// }
// fn slow(name: &str, ms: u64) {
//     thread::sleep(Duration::from_millis(ms));
//     println!("'{name}' ran for {ms}ms");
// }
// fn main() {
//     trpl::run(async {
//         let a = async {
//             println!("'a' started.");
//             slow("a", 30);
//             trpl::yield_now().await;
//             slow("a", 10);
//             trpl::yield_now().await;
//             slow("a", 20);
//             trpl::yield_now().await;
//             println!("'a' finished.");
//         };
//
//         let b = async {
//             println!("'b' started.");
//             slow("b", 75);
//             trpl::yield_now().await;
//             slow("b", 10);
//             trpl::yield_now().await;
//             slow("b", 15);
//             trpl::yield_now().await;
//             slow("b", 35);
//             trpl::yield_now().await;
//             println!("'b' finished.");
//         };
//
//         trpl::race(a, b).await;
//     })
// }
use trpl::{Stream, StreamExt, ReceiverStream};
use std::{pin::pin, time::Duration};
// fn main() {
//     trpl::run(async{
//         let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//         let iter = values.iter().map(|n| n * 2);
//         let mut stream = trpl::stream_from_iter(iter);
//
//         while let Some(value) = stream.next().await {
//             println!("The value was: {value}");
//         }
//     });
// }
fn main() {
    trpl::run(async {
        //let mut messages = get_messages();
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));
        let intervals = get_intervals();
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);
        while let Some(result) = stream.next().await {
            match result {
                Ok(msg) => println!("msg: {}", msg),
                Err(e) => println!("Problem: {e:?}")
            }
        }
    });
}
// fn get_messages() ->impl Stream<Item = String> {
//     let (tx, rx) = trpl::channel();
//     let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
//
//     for message in messages {
//         tx.send(format!("Message: {}", message)).unwrap();
//     }
//     ReceiverStream::new(rx)
// }

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    trpl::spawn_task(async move {

        for (index, message) in messages.into_iter().enumerate() {
            println!("[{}] {}", index, message);
            let time_to_sleep = if index % 2 == 0 {100} else {300};
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

