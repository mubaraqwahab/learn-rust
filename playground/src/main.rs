#![allow(unused)]

mod examples;

use examples::hello_async::*;
use trpl::Either;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let fut1 = page_title(&args[1]);
        let fut2 = page_title(&args[2]);

        let (url, title) = match trpl::race(fut1, fut2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match title {
            Some(title) => println!("Its page title is: {title}"),
            None => println!("Its page title couldn't be parsed"),
        }
    });
}
