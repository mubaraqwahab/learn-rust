use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn tx_rx() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = ["hi", "from", "the", "thread"];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        for received in rx {
            println!("Got: {received}");
        }
    });

    handle2.join().unwrap();
}
