use std::env::{args, Args};

fn main() {
    let mut arguments: Args = args();
    let proc = arguments.next().unwrap();
    let first = arguments.next().unwrap();
    let operator = arguments.next().unwrap();
    let second = arguments.next().unwrap();

    println!("{:?}", first);
    println!("{:?}", operator);
    println!("{:?}", second);
}
