use std::env::{args, Args};

fn main() {
    let mut arguments: Args = args();
    let _proc = arguments.next().unwrap();
    let first = arguments.next().unwrap();
    let operator = arguments.next().unwrap().chars().next().unwrap();
    let second = arguments.next().unwrap();

    let first_operand = first.parse::<f32>().unwrap();
    let second_operand = second.parse::<f32>().unwrap();

    println!("{} {} {} = {}", first, operator, second, calculate(first_operand, second_operand, operator));
}

fn calculate(first: f32, second: f32, operator: char) -> f32 {
    match operator {
        '+' => first + second,
        '-' => first - second,
        '*' => first * second,
        '/' => first / second,
        _ => 0.0,
    }
}
