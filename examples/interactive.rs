use std::io::{self, Write};

extern crate number_shortener;

use number_shortener::{shorten_number};

fn read_input() -> String {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Failed to read from console");

    buffer.trim().to_owned()
}

fn main() {
    print!("Enter a number to shorten: ");

    io::stdout().flush().unwrap();

    let num: f32 = read_input().parse::<f32>().expect("Not a valid number");

    println!("Number to shorten: {}", num);

    let short_number = shorten_number(num);

    println!("Shortened number is: {:?}", short_number);
}
