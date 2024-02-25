use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug)]
struct Circle {
    _radius: f64,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {item:?}");
}

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, input_1: U, input_2: U) {
    println!(
        "{statement}! Is {input_1} greater than {input_2}? {}",
        input_1 > input_2
    );
}

fn main() {
    print_item(5);
    print_item("Hello, world!");
    print_item(Circle { _radius: 2.0 });
    compare_and_display("Listen up!", 9, 8);
}
