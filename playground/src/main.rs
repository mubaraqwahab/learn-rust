#![allow(unused)]

mod examples;

use examples::hello_unsafe;
use trpl::Either;

fn main() {
    hello_unsafe::run_unsafe();
}
