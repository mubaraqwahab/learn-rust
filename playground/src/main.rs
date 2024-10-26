#![allow(unused)]

mod examples;

use core::error;
use examples::game::*;
use examples::traits::{NewsArticle, Other, Summary, Tweet};
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::thread;

fn main() {
    // let myfile_result = File::open("./README.txt");
    // let myfile = match myfile_result {
    //     Ok(file) => file,
    //     // Err(error:std::io::Error(ErrorKind::NotFound)) => panic!("file not found: {error:?}"),
    //     Err(error) => panic!("error opening file: {error:?}"),
    // };

    // let n = NewsArticle {
    //     headline: String::from("hi"),
    //     author: String::from("hi"),
    //     content: String::from("hi there"),
    //     location: String::from(""),
    // };
    // // n.summarize();
    // println!("{}", Other::summarize(&n));

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

fn round_in_place(v: &mut Vec<f64>) {
    for n in v {
        *n = n.round();
    }
}

fn give_and_take(_v: &mut Vec<i32>) {}
