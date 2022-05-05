use std::env::{args, Args};

fn main() {
    let mut arguments: Args = args();
    let _proc = arguments.next().unwrap();
    let first = arguments.next().unwrap();
    let operator = arguments.next().unwrap();
    let second = arguments.next().unwrap();

    let first_operand = first.parse::<f32>().unwrap();
    let second_operand = second.parse::<f32>().unwrap();

    println!("{} {} {}", first, operator, second);
}
