// This is a code from the Rust training example.  Modify it to remove
// channel (mpsc) use completely, create some shared DataManager and
// make use of it in both threads. Make use of Arc and Mutex.
//
// Remove first and last line inside *for* loop from each thread code
// and all the other code related to channel use. Beyond that there
// shouldn't be more changes in the threads, except variable names.
//
// The commented code is a suggestion for implementation start.

struct DataManager {
    // ...Vec...
}

impl DataManager {
    //fn new()

    //fn push(...)

    //fn remove(...)
}


use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx2) = mpsc::channel();
    let (tx2, rx1) = mpsc::channel();
    let (tx, rx) = mpsc::channel();

    let data = vec!["some".to_string(),
                    "data".to_string()];

    tx2.send(data).unwrap();

    //let data = ...DataManager...
    //data.push("some".to_string());
    //data.push("data".to_string());

    let thread1 = move || {
        for i in 1..5 {
            let mut data = rx1.recv().unwrap();
            data.push("thread1 data".to_string());
            if i == 2 { data.remove(1); }
            thread::sleep(Duration::from_millis(10));
            tx1.send(data).unwrap();
        }
        let data: Vec<String> = rx1.recv().unwrap();
        tx.send(data).unwrap();
    };

    let thread2 = move || {
        for i in 1..5 {
            let mut data = rx2.recv().unwrap();
            data.push("thread2 data".to_string());
            if i == 1 { data.remove(2); }
            thread::sleep(Duration::from_millis(20));
            tx2.send(data).unwrap();
        }
    };

    let h1 = thread::spawn(thread1);
    let h2 = thread::spawn(thread2);

    let data = rx.recv().unwrap();
    println!("{:?}", data);

    for h in [h1, h2] {
        h.join().unwrap();
    }
}
