#![allow(unused)]

mod examples;

use core::error;
use examples::game::*;
use examples::traits::{NewsArticle, Other, Summary, Tweet};
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::sync::mpsc::{self, SendError};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    tx.clone();

    let handle = thread::spawn(move || {
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
