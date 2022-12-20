use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi from number {} in the spawned thread", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("hi from number {} in the main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
