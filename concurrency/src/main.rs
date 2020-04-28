use std::thread;
use std::time::Duration;
//for channel in rust
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    let v = vec![1, 2, 3];

    let handle_vec = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle_vec.join().unwrap();

    //sending single message

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let value = String::from("trans message");
        tx.send(value).unwrap();
    });
    let recieved = rx.recv().unwrap();
    println!("I got this message from channel: {}", recieved);

    //sending multiple messages at once
    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move || {
        let value = vec![
            String::from("strin1"),
            String::from("string2"),
            String::from("string3"),
        ];
        for item in value {
            tx2.send(item).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved_value in rx2 {
        println!("got this val:{}", recieved_value);
    }

    //creating multiple producers
    let (tx3, rx3) = mpsc::channel();

    let tx4 = mpsc::Sender::clone(&tx3);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx4.send(val).unwrap();
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
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx3 {
        println!("Got: {}", received);
    }

    //using mutex which allows internal mutability
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        //using arc instead of rc
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
